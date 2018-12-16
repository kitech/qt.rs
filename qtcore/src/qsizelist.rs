
// mod ::core::QSize
// package qtcore
// /usr/include/qt/QtCore/qsize.h
// #include <qsize.h>
// #include <QtCore>

//  header block end

//  ext block begin

/*
#include <stdlib.h>
// extern C begin: 0
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin
pub struct QSizeList {
    pub qclsinst: usize /* *mut c_void*/,
}
// QList<T> & operator=(const QList<T> &)
impl QSizeList {
pub fn Operator_equal_0(&self) -> usize {
    // QSizeList_operator_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_operator_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator=(QList<T> &&)
impl QSizeList {
pub fn Operator_equal_1(&self) -> usize {
    // QSizeList_operator_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_operator_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void swap(QList<T> &)
impl QSizeList {
pub fn Swap_0(&self) -> (/*void*/) {
    // QSizeList_swap_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_swap_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool operator==(const QList<T> &)
impl QSizeList {
pub fn Operator_equal_equal_0(&self) -> bool {
    // QSizeList_operator_equal_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_operator_equal_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool operator!=(const QList<T> &)
impl QSizeList {
pub fn Operator_not_equal_0(&self) -> bool {
    // QSizeList_operator_not_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_operator_not_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int size()
impl QSizeList {
pub fn Size_0(&self) -> i32 {
    // QSizeList_size_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_size_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// void detach()
impl QSizeList {
pub fn Detach_0(&self) -> (/*void*/) {
    // QSizeList_detach_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_detach_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detachShared()
impl QSizeList {
pub fn DetachShared_0(&self) -> (/*void*/) {
    // QSizeList_detachShared_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_detachShared_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isDetached()
impl QSizeList {
pub fn IsDetached_0(&self) -> bool {
    // QSizeList_isDetached_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_isDetached_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void setSharable(bool)
impl QSizeList {
pub fn SetSharable_0(&self) -> (/*void*/) {
    // QSizeList_setSharable_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_setSharable_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isSharedWith(const QList<T> &)
impl QSizeList {
pub fn IsSharedWith_0(&self) -> bool {
    // QSizeList_isSharedWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_isSharedWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool isEmpty()
impl QSizeList {
pub fn IsEmpty_0(&self) -> bool {
    // QSizeList_isEmpty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_isEmpty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void clear()
impl QSizeList {
pub fn Clear_0(&self) -> (/*void*/) {
    // QSizeList_clear_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_clear_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// const T & at(int)
impl QSizeList {
pub fn At_0(&self) -> usize {
    // QSizeList_at_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_at_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & operator[](int)
impl QSizeList {
pub fn Operator_get_index_0(&self) -> usize {
    // QSizeList_operator_get_index_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_operator_get_index_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & operator[](int)
impl QSizeList {
pub fn Operator_get_index_1(&self) -> usize {
    // QSizeList_operator_get_index_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_operator_get_index_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void reserve(int)
impl QSizeList {
pub fn Reserve_0(&self) -> (/*void*/) {
    // QSizeList_reserve_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_reserve_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void append(const T &)
impl QSizeList {
pub fn Append_0(&self) -> (/*void*/) {
    // QSizeList_append_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_append_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void append(const QList<T> &)
impl QSizeList {
pub fn Append_1(&self) -> (/*void*/) {
    // QSizeList_append_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_append_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void prepend(const T &)
impl QSizeList {
pub fn Prepend_0(&self) -> (/*void*/) {
    // QSizeList_prepend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_prepend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void insert(int, const T &)
impl QSizeList {
pub fn Insert_0(&self) -> (/*void*/) {
    // QSizeList_insert_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_insert_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void replace(int, const T &)
impl QSizeList {
pub fn Replace_0(&self) -> (/*void*/) {
    // QSizeList_replace_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_replace_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void removeAt(int)
impl QSizeList {
pub fn RemoveAt_0(&self) -> (/*void*/) {
    // QSizeList_removeAt_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_removeAt_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int removeAll(const T &)
impl QSizeList {
pub fn RemoveAll_0(&self) -> i32 {
    // QSizeList_removeAll_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_removeAll_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool removeOne(const T &)
impl QSizeList {
pub fn RemoveOne_0(&self) -> bool {
    // QSizeList_removeOne_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_removeOne_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// T takeAt(int)
impl QSizeList {
pub fn TakeAt_0(&self) -> usize {
    // QSizeList_takeAt_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_takeAt_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T takeFirst()
impl QSizeList {
pub fn TakeFirst_0(&self) -> usize {
    // QSizeList_takeFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_takeFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T takeLast()
impl QSizeList {
pub fn TakeLast_0(&self) -> usize {
    // QSizeList_takeLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_takeLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void move(int, int)
impl QSizeList {
pub fn Move_0(&self) -> (/*void*/) {
    // QSizeList_move_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_move_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void swap(int, int)
impl QSizeList {
pub fn Swap_1(&self) -> (/*void*/) {
    // QSizeList_swap_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_swap_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int indexOf(const T &, int)
impl QSizeList {
pub fn IndexOf_0(&self) -> i32 {
    // QSizeList_indexOf_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_indexOf_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int lastIndexOf(const T &, int)
impl QSizeList {
pub fn LastIndexOf_0(&self) -> i32 {
    // QSizeList_lastIndexOf_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_lastIndexOf_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool contains(const T &)
impl QSizeList {
pub fn Contains_0(&self) -> bool {
    // QSizeList_contains_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_contains_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int count(const T &)
impl QSizeList {
pub fn Count_0(&self) -> i32 {
    // QSizeList_count_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_count_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// QList::iterator begin()
impl QSizeList {
pub fn Begin_0(&self) -> usize {
    // QSizeList_begin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_begin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator begin()
impl QSizeList {
pub fn Begin_1(&self) -> usize {
    // QSizeList_begin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_begin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator cbegin()
impl QSizeList {
pub fn Cbegin_0(&self) -> usize {
    // QSizeList_cbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_cbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator constBegin()
impl QSizeList {
pub fn ConstBegin_0(&self) -> usize {
    // QSizeList_constBegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_constBegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator end()
impl QSizeList {
pub fn End_0(&self) -> usize {
    // QSizeList_end_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_end_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator end()
impl QSizeList {
pub fn End_1(&self) -> usize {
    // QSizeList_end_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_end_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator cend()
impl QSizeList {
pub fn Cend_0(&self) -> usize {
    // QSizeList_cend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_cend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator constEnd()
impl QSizeList {
pub fn ConstEnd_0(&self) -> usize {
    // QSizeList_constEnd_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_constEnd_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::reverse_iterator rbegin()
impl QSizeList {
pub fn Rbegin_0(&self) -> usize {
    // QSizeList_rbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_rbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::reverse_iterator rend()
impl QSizeList {
pub fn Rend_0(&self) -> usize {
    // QSizeList_rend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_rend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator rbegin()
impl QSizeList {
pub fn Rbegin_1(&self) -> usize {
    // QSizeList_rbegin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_rbegin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator rend()
impl QSizeList {
pub fn Rend_1(&self) -> usize {
    // QSizeList_rend_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_rend_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator crbegin()
impl QSizeList {
pub fn Crbegin_0(&self) -> usize {
    // QSizeList_crbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_crbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator crend()
impl QSizeList {
pub fn Crend_0(&self) -> usize {
    // QSizeList_crend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_crend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator insert(QList::iterator, const T &)
impl QSizeList {
pub fn Insert_1(&self) -> usize {
    // QSizeList_insert_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_insert_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator erase(QList::iterator)
impl QSizeList {
pub fn Erase_0(&self) -> usize {
    // QSizeList_erase_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_erase_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator erase(QList::iterator, QList::iterator)
impl QSizeList {
pub fn Erase_1(&self) -> usize {
    // QSizeList_erase_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_erase_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// int count()
impl QSizeList {
pub fn Count_1(&self) -> i32 {
    // QSizeList_count_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_count_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int length()
impl QSizeList {
pub fn Length_0(&self) -> i32 {
    // QSizeList_length_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_length_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// T & first()
impl QSizeList {
pub fn First_0(&self) -> usize {
    // QSizeList_first_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_first_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & constFirst()
impl QSizeList {
pub fn ConstFirst_0(&self) -> usize {
    // QSizeList_constFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_constFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & first()
impl QSizeList {
pub fn First_1(&self) -> usize {
    // QSizeList_first_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_first_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & last()
impl QSizeList {
pub fn Last_0(&self) -> usize {
    // QSizeList_last_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_last_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & last()
impl QSizeList {
pub fn Last_1(&self) -> usize {
    // QSizeList_last_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_last_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & constLast()
impl QSizeList {
pub fn ConstLast_0(&self) -> usize {
    // QSizeList_constLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_constLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void removeFirst()
impl QSizeList {
pub fn RemoveFirst_0(&self) -> (/*void*/) {
    // QSizeList_removeFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_removeFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void removeLast()
impl QSizeList {
pub fn RemoveLast_0(&self) -> (/*void*/) {
    // QSizeList_removeLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_removeLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool startsWith(const T &)
impl QSizeList {
pub fn StartsWith_0(&self) -> bool {
    // QSizeList_startsWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_startsWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool endsWith(const T &)
impl QSizeList {
pub fn EndsWith_0(&self) -> bool {
    // QSizeList_endsWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_endsWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QList<T> mid(int, int)
impl QSizeList {
pub fn Mid_0(&self) -> usize {
    // QSizeList_mid_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_mid_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T value(int)
impl QSizeList {
pub fn Value_0(&self) -> usize {
    // QSizeList_value_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_value_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T value(int, const T &)
impl QSizeList {
pub fn Value_1(&self) -> usize {
    // QSizeList_value_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_value_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void push_back(const T &)
impl QSizeList {
pub fn Push_back_0(&self) -> (/*void*/) {
    // QSizeList_push_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_push_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void push_front(const T &)
impl QSizeList {
pub fn Push_front_0(&self) -> (/*void*/) {
    // QSizeList_push_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_push_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// T & front()
impl QSizeList {
pub fn Front_0(&self) -> usize {
    // QSizeList_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & front()
impl QSizeList {
pub fn Front_1(&self) -> usize {
    // QSizeList_front_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_front_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & back()
impl QSizeList {
pub fn Back_0(&self) -> usize {
    // QSizeList_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & back()
impl QSizeList {
pub fn Back_1(&self) -> usize {
    // QSizeList_back_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_back_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void pop_front()
impl QSizeList {
pub fn Pop_front_0(&self) -> (/*void*/) {
    // QSizeList_pop_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_pop_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void pop_back()
impl QSizeList {
pub fn Pop_back_0(&self) -> (/*void*/) {
    // QSizeList_pop_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_pop_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool empty()
impl QSizeList {
pub fn Empty_0(&self) -> bool {
    // QSizeList_empty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_empty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QList<T> & operator+=(const QList<T> &)
impl QSizeList {
pub fn Operator_add_equal_0(&self) -> usize {
    // QSizeList_operator_add_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_operator_add_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> operator+(const QList<T> &)
impl QSizeList {
pub fn Operator_add_0(&self) -> usize {
    // QSizeList_operator_add_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_operator_add_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator+=(const T &)
impl QSizeList {
pub fn Operator_add_equal_1(&self) -> usize {
    // QSizeList_operator_add_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_operator_add_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator<<(const T &)
impl QSizeList {
pub fn Operator_left_shift_0(&self) -> usize {
    // QSizeList_operator_left_shift_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_operator_left_shift_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator<<(const QList<T> &)
impl QSizeList {
pub fn Operator_left_shift_1(&self) -> usize {
    // QSizeList_operator_left_shift_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_operator_left_shift_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QVector<T> toVector()
impl QSizeList {
pub fn ToVector_0(&self) -> usize {
    // QSizeList_toVector_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_toVector_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> toSet()
impl QSizeList {
pub fn ToSet_0(&self) -> usize {
    // QSizeList_toSet_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_toSet_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromVector(const QVector<T> &)
impl QSizeList {
pub fn FromVector_0(&self) -> usize {
    // QSizeList_fromVector_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_fromVector_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromSet(const QSet<T> &)
impl QSizeList {
pub fn FromSet_0(&self) -> usize {
    // QSizeList_fromSet_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_fromSet_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromStdList(const std::list<T> &)
impl QSizeList {
pub fn FromStdList_0(&self) -> usize {
    // QSizeList_fromStdList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_fromStdList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// std::list<T> toStdList()
impl QSizeList {
pub fn ToStdList_0(&self) -> i32 {
    // QSizeList_toStdList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_toStdList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :i32 = Default::default();
    return dret;
  }
}
// QList::Node * detach_helper_grow(int, int)
impl QSizeList {
pub fn Detach_helper_grow_0(&self) -> usize {
    // QSizeList_detach_helper_grow_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_detach_helper_grow_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void detach_helper(int)
impl QSizeList {
pub fn Detach_helper_0(&self) -> (/*void*/) {
    // QSizeList_detach_helper_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_detach_helper_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detach_helper()
impl QSizeList {
pub fn Detach_helper_1(&self) -> (/*void*/) {
    // QSizeList_detach_helper_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_detach_helper_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void dealloc(QListData::Data *)
impl QSizeList {
pub fn Dealloc_0(&self) -> (/*void*/) {
    // QSizeList_dealloc_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_dealloc_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_construct(QList::Node *, const T &)
impl QSizeList {
pub fn Node_construct_0(&self) -> (/*void*/) {
    // QSizeList_node_construct_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_node_construct_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_destruct(QList::Node *)
impl QSizeList {
pub fn Node_destruct_0(&self) -> (/*void*/) {
    // QSizeList_node_destruct_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_node_destruct_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_copy(QList::Node *, QList::Node *, QList::Node *)
impl QSizeList {
pub fn Node_copy_0(&self) -> (/*void*/) {
    // QSizeList_node_copy_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_node_copy_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_destruct(QList::Node *, QList::Node *)
impl QSizeList {
pub fn Node_destruct_1(&self) -> (/*void*/) {
    // QSizeList_node_destruct_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_node_destruct_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isValidIterator(const QList::iterator &)
impl QSizeList {
pub fn IsValidIterator_0(&self) -> bool {
    // QSizeList_isValidIterator_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_isValidIterator_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool op_eq_impl(const QList<T> &, QListData::NotArrayCompatibleLayout)
impl QSizeList {
pub fn Op_eq_impl_0(&self) -> bool {
    // QSizeList_op_eq_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_op_eq_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool op_eq_impl(const QList<T> &, QListData::ArrayCompatibleLayout)
impl QSizeList {
pub fn Op_eq_impl_1(&self) -> bool {
    // QSizeList_op_eq_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_op_eq_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains_impl(const T &, QListData::NotArrayCompatibleLayout)
impl QSizeList {
pub fn Contains_impl_0(&self) -> bool {
    // QSizeList_contains_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_contains_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains_impl(const T &, QListData::ArrayCompatibleLayout)
impl QSizeList {
pub fn Contains_impl_1(&self) -> bool {
    // QSizeList_contains_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_contains_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int count_impl(const T &, QListData::NotArrayCompatibleLayout)
impl QSizeList {
pub fn Count_impl_0(&self) -> i32 {
    // QSizeList_count_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_count_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int count_impl(const T &, QListData::ArrayCompatibleLayout)
impl QSizeList {
pub fn Count_impl_1(&self) -> i32 {
    // QSizeList_count_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QSizeList_count_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
//  body block end
