# 数据库原题（座机） 答案 - 持续更新

## 选择题答案


| 30   | 概念模型 | C    | ？D？ |      |
| ---- | -------- | ---- | ----- | ---- |
| B    | B        | D    | B     |      |
| B    | C        |      |       |      |


## 大题答案

1.
```sql
# 每个书架的书 -》 每个房间所有书架的书  
with A as(
select rID,sum(sumA) as total
from shelf natural join ( select sID,count(*) as sumA
from  book group by sID)
group by rID
)
select rName , total 
from rID natural join A
where total > 2000
```



2. "NOT EXISTS" find the press who has no books stored in room "A"  For each press as such , following information should be listed 

* name of the press

```sql
select pName
from press
where not exists (select * from book natural join shelf natural join room
                 where room='A' AND book.pID=press.pID )
```



3. find total number of books provided by each press listed in table 'Press', even if a press doesn't provide any books to the libary . For each press as such, following information should be listed:

* Name of  the press
* Total number of the books provided by the press

```sql
select pName , nvl(total,0)
from press left join (select pID,count(*) from book group by pID) on press.pID = book.pID 
```



4. 'Scalar subquery' 

For each room, find information listed below:

* Name of the room
* Number of presses whose books stored in the room
* total salary of librarian who manage shelves in the room

```sql
select rName , (select sum(c) from shelf natural join (select sID,count(*) from book group by sID) where shelf.rID = room.rID ),
(select sum(nBal) from librarian natural join shelf where shelf.rID=room.rID)
from room
```



5. delete all books provided by the press "HM"

```sql
delete from book
where pID = (select pID from press where pName='HM' )
```



6. "...NOT EXITS...EXCEPT..."  

   find presses that each room has books provided by the press . The information should involve: 

   * Name of the press 
