#[derive(Default)]
/// 某个线程的内存映射关系
pub struct Mapping {
    /// 保存所有使用到的页表
    page_tables: Vec<PageTableTracker>,
    /// 根页表的物理页号
    root_ppn: PhysicalPageNumber,
    /// 所有分配的物理页面映射信息
    mapped_pairs: VecDeque<(VirtualPageNumber, FrameTracker)>,
}

impl Mapping {
    /// 创建一个有根节点的映射
    pub fn new() -> MemoryResult<Mapping> {
        let root_table = PageTableTracker::new(FRAME_ALLOCATOR.lock().alloc()?);
        let root_ppn = root_table.page_number();
        Ok(Mapping {
            page_tables: vec![root_table],
            root_ppn,
            mapped_pairs: VecDeque::new(),
        })
    }

    /// 找到给定虚拟页号的三级页表项
    ///
    /// 如果找不到对应的页表项，则会相应创建页表
    pub fn find_entry(&mut self, vpn: VirtualPageNumber) -> MemoryResult<&mut PageTableEntry> {
        // 从根页表开始向下查询
        // 这里不用 self.page_tables[0] 避免后面产生 borrow-check 冲突（我太菜了）
        let root_table: &mut PageTable = PhysicalAddress::from(self.root_ppn).deref_kernel();
        let mut entry = &mut root_table.entries[vpn.levels()[0]];
        for vpn_slice in &vpn.levels()[1..] {
            if entry.is_empty() {
                // 如果页表不存在，则需要分配一个新的页表
                let new_table = PageTableTracker::new(FRAME_ALLOCATOR.lock().alloc()?);
                let new_ppn = new_table.page_number();
                // 将新页表的页号写入当前的页表项
                *entry = PageTableEntry::new(Some(new_ppn), Flags::VALID);
                // 保存页表
                self.page_tables.push(new_table);
            }
            // 进入下一级页表（使用偏移量来访问物理地址）
            entry = &mut entry.get_next_table().entries[*vpn_slice];
        }
        // 此时 entry 位于第三级页表
        Ok(entry)
    }

    /// 为给定的虚拟 / 物理页号建立映射关系
    fn map_one(
        &mut self,
        vpn: VirtualPageNumber,
        ppn: Option<PhysicalPageNumber>,
        flags: Flags,
    ) -> MemoryResult<()> {
        // 定位到页表项
        let entry = self.find_entry(vpn)?;
        assert!(entry.is_empty(), "virtual address is already mapped");
        // 页表项为空，则写入内容
        *entry = PageTableEntry::new(ppn, flags);
        Ok(())
    }

    /// 加入一段映射，可能会相应地分配物理页面
    ///
    /// 未被分配物理页面的虚拟页号暂时不会写入页表当中，它们会在发生 PageFault 后再建立页表项。
    pub fn map(&mut self, segment: &Segment, init_data: Option<&[u8]>) -> MemoryResult<()> {
        match segment.map_type {
            // 线性映射，直接对虚拟地址进行转换
            MapType::Linear => {
                for vpn in segment.page_range().iter() {
                    self.map_one(vpn, Some(vpn.into()), segment.flags)?;
                }
                // 拷贝数据
                if let Some(data) = init_data {
                    unsafe {
                        (&mut *slice_from_raw_parts_mut(segment.range.start.deref(), data.len()))
                            .copy_from_slice(data);
                    }
                }
            }
            // 需要分配帧进行映射
            MapType::Framed => {
                for vpn in segment.page_range().iter() {
                    // 页面的数据，默认为全零
                    let mut page_data = [0u8; PAGE_SIZE];
                    // 如果提供了数据，则使用这些数据来填充 page_data
                    if let Some(init_data) = init_data {
                        if !init_data.is_empty() {
                            // 这里必须进行一些调整，因为传入的数据可能并非按照整页对齐

                            // 拷贝时必须考虑区间与整页不对齐的情况
                            //    start（仅第一页时非零）
                            //      |        stop（仅最后一页时非零）
                            // 0    |---data---|          4096
                            // |------------page------------|
                            let page_address = VirtualAddress::from(vpn);
                            let start = if segment.range.start > page_address {
                                segment.range.start - page_address
                            } else {
                                0
                            };
                            let stop = min(PAGE_SIZE, segment.range.end - page_address);
                            // 计算来源和目标区间并进行拷贝
                            let dst_slice = &mut page_data[start..stop];
                            let src_slice = &init_data[(page_address + start - segment.range.start)
                                ..(page_address + stop - segment.range.start)];
                            dst_slice.copy_from_slice(src_slice);
                        }
                    };

                    // 建立映射
                    let mut frame = FRAME_ALLOCATOR.lock().alloc()?;
                    // 更新页表
                    self.map_one(vpn, Some(frame.page_number()), segment.flags)?;
                    // 写入数据
                    (*frame).copy_from_slice(&page_data);
                    // 保存
                    self.mapped_pairs.push_back((vpn, frame));
                }
            }
        }
        Ok(())
    }

    /// 将当前的映射加载到 `satp` 寄存器
    pub fn activate(&self) {
        // satp 低 27 位为页号，高 4 位为模式，8 表示 Sv39
        let new_satp = self.root_ppn.0 | (8 << 60);
        unsafe {
            // 将 new_satp 的值写到 satp 寄存器
            llvm_asm!("csrw satp, $0" :: "r"(new_satp) :: "volatile");
            // 刷新 TLB
            llvm_asm!("sfence.vma" :::: "volatile");
        }
    }
}
