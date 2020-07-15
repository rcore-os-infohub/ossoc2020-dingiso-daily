/* 目标架构 */
OUTPUT_ARCH(riscv)

/*执行入口*/
ENTRY(_start)

/*数据存放起始地址*/
BASE_ADDRESS = 0x80200000;

SECTIONS
{
	/* .表示当前地址（location counter) */
	. = BASE_ADDRESS;
	
	/* start 符号表示全部的开始位置*/
	kernel_start = .;

	text_start = .;

	/* .text 字段 */
	.text : {
		/* 把entry函数放在最前面*/
		*(.text.entry)
		/*要链接的文件的 .text 字段集中放在这里 */
		*(.text .text.*)
	}

	rodata_start = .;

	/*.rodatt 字段*/
	.rodata :{
		/*要链接的文件的 .rodata字段集中放在这里*/
		*(.rodata .rodata.*)
	}

	data_start = .;

	/* .data 字段*/
	.data :{
		*(.data .data.*)
	}

	bss_start = .;

	.bss : {
		*(.sbss .bss .bss.*)
	}

	kernel_end = .;
}
