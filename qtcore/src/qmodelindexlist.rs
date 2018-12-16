
// mod ::core::QModelIndex
// package qtcore
// /usr/include/qt/QtCore/qabstractitemmodel.h
// #include <qabstractitemmodel.h>
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
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin
pub struct QModelIndexList {
    pub qclsinst: usize /* *mut c_void*/,
}
// QList<T> & operator=(const QList<T> &)
impl QModelIndexList {
pub fn Operator_equal_0(&self) -> usize {
    // QModelIndexList_operator_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_operator_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator=(QList<T> &&)
impl QModelIndexList {
pub fn Operator_equal_1(&self) -> usize {
    // QModelIndexList_operator_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_operator_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void swap(QList<T> &)
impl QModelIndexList {
pub fn Swap_0(&self) -> (/*void*/) {
    // QModelIndexList_swap_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_swap_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool operator==(const QList<T> &)
impl QModelIndexList {
pub fn Operator_equal_equal_0(&self) -> bool {
    // QModelIndexList_operator_equal_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_operator_equal_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool operator!=(const QList<T> &)
impl QModelIndexList {
pub fn Operator_not_equal_0(&self) -> bool {
    // QModelIndexList_operator_not_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_operator_not_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int size()
impl QModelIndexList {
pub fn Size_0(&self) -> i32 {
    // QModelIndexList_size_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_size_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// void detach()
impl QModelIndexList {
pub fn Detach_0(&self) -> (/*void*/) {
    // QModelIndexList_detach_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_detach_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detachShared()
impl QModelIndexList {
pub fn DetachShared_0(&self) -> (/*void*/) {
    // QModelIndexList_detachShared_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_detachShared_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isDetached()
impl QModelIndexList {
pub fn IsDetached_0(&self) -> bool {
    // QModelIndexList_isDetached_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_isDetached_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void setSharable(bool)
impl QModelIndexList {
pub fn SetSharable_0(&self) -> (/*void*/) {
    // QModelIndexList_setSharable_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_setSharable_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isSharedWith(const QList<T> &)
impl QModelIndexList {
pub fn IsSharedWith_0(&self) -> bool {
    // QModelIndexList_isSharedWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_isSharedWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool isEmpty()
impl QModelIndexList {
pub fn IsEmpty_0(&self) -> bool {
    // QModelIndexList_isEmpty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_isEmpty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void clear()
impl QModelIndexList {
pub fn Clear_0(&self) -> (/*void*/) {
    // QModelIndexList_clear_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_clear_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// const T & at(int)
impl QModelIndexList {
pub fn At_0(&self) -> usize {
    // QModelIndexList_at_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_at_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & operator[](int)
impl QModelIndexList {
pub fn Operator_get_index_0(&self) -> usize {
    // QModelIndexList_operator_get_index_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_operator_get_index_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & operator[](int)
impl QModelIndexList {
pub fn Operator_get_index_1(&self) -> usize {
    // QModelIndexList_operator_get_index_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_operator_get_index_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void reserve(int)
impl QModelIndexList {
pub fn Reserve_0(&self) -> (/*void*/) {
    // QModelIndexList_reserve_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_reserve_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void append(const T &)
impl QModelIndexList {
pub fn Append_0(&self) -> (/*void*/) {
    // QModelIndexList_append_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_append_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void append(const QList<T> &)
impl QModelIndexList {
pub fn Append_1(&self) -> (/*void*/) {
    // QModelIndexList_append_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_append_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void prepend(const T &)
impl QModelIndexList {
pub fn Prepend_0(&self) -> (/*void*/) {
    // QModelIndexList_prepend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_prepend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void insert(int, const T &)
impl QModelIndexList {
pub fn Insert_0(&self) -> (/*void*/) {
    // QModelIndexList_insert_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_insert_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void replace(int, const T &)
impl QModelIndexList {
pub fn Replace_0(&self) -> (/*void*/) {
    // QModelIndexList_replace_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_replace_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void removeAt(int)
impl QModelIndexList {
pub fn RemoveAt_0(&self) -> (/*void*/) {
    // QModelIndexList_removeAt_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_removeAt_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int removeAll(const T &)
impl QModelIndexList {
pub fn RemoveAll_0(&self) -> i32 {
    // QModelIndexList_removeAll_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_removeAll_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool removeOne(const T &)
impl QModelIndexList {
pub fn RemoveOne_0(&self) -> bool {
    // QModelIndexList_removeOne_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_removeOne_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// T takeAt(int)
impl QModelIndexList {
pub fn TakeAt_0(&self) -> usize {
    // QModelIndexList_takeAt_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_takeAt_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T takeFirst()
impl QModelIndexList {
pub fn TakeFirst_0(&self) -> usize {
    // QModelIndexList_takeFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_takeFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T takeLast()
impl QModelIndexList {
pub fn TakeLast_0(&self) -> usize {
    // QModelIndexList_takeLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_takeLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void move(int, int)
impl QModelIndexList {
pub fn Move_0(&self) -> (/*void*/) {
    // QModelIndexList_move_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_move_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void swap(int, int)
impl QModelIndexList {
pub fn Swap_1(&self) -> (/*void*/) {
    // QModelIndexList_swap_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_swap_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int indexOf(const T &, int)
impl QModelIndexList {
pub fn IndexOf_0(&self) -> i32 {
    // QModelIndexList_indexOf_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_indexOf_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int lastIndexOf(const T &, int)
impl QModelIndexList {
pub fn LastIndexOf_0(&self) -> i32 {
    // QModelIndexList_lastIndexOf_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_lastIndexOf_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool contains(const T &)
impl QModelIndexList {
pub fn Contains_0(&self) -> bool {
    // QModelIndexList_contains_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_contains_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int count(const T &)
impl QModelIndexList {
pub fn Count_0(&self) -> i32 {
    // QModelIndexList_count_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_count_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// QList::iterator begin()
impl QModelIndexList {
pub fn Begin_0(&self) -> usize {
    // QModelIndexList_begin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_begin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator begin()
impl QModelIndexList {
pub fn Begin_1(&self) -> usize {
    // QModelIndexList_begin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_begin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator cbegin()
impl QModelIndexList {
pub fn Cbegin_0(&self) -> usize {
    // QModelIndexList_cbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_cbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator constBegin()
impl QModelIndexList {
pub fn ConstBegin_0(&self) -> usize {
    // QModelIndexList_constBegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_constBegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator end()
impl QModelIndexList {
pub fn End_0(&self) -> usize {
    // QModelIndexList_end_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_end_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator end()
impl QModelIndexList {
pub fn End_1(&self) -> usize {
    // QModelIndexList_end_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_end_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator cend()
impl QModelIndexList {
pub fn Cend_0(&self) -> usize {
    // QModelIndexList_cend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_cend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator constEnd()
impl QModelIndexList {
pub fn ConstEnd_0(&self) -> usize {
    // QModelIndexList_constEnd_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_constEnd_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::reverse_iterator rbegin()
impl QModelIndexList {
pub fn Rbegin_0(&self) -> usize {
    // QModelIndexList_rbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_rbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::reverse_iterator rend()
impl QModelIndexList {
pub fn Rend_0(&self) -> usize {
    // QModelIndexList_rend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_rend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator rbegin()
impl QModelIndexList {
pub fn Rbegin_1(&self) -> usize {
    // QModelIndexList_rbegin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_rbegin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator rend()
impl QModelIndexList {
pub fn Rend_1(&self) -> usize {
    // QModelIndexList_rend_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_rend_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator crbegin()
impl QModelIndexList {
pub fn Crbegin_0(&self) -> usize {
    // QModelIndexList_crbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_crbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator crend()
impl QModelIndexList {
pub fn Crend_0(&self) -> usize {
    // QModelIndexList_crend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_crend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator insert(QList::iterator, const T &)
impl QModelIndexList {
pub fn Insert_1(&self) -> usize {
    // QModelIndexList_insert_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_insert_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator erase(QList::iterator)
impl QModelIndexList {
pub fn Erase_0(&self) -> usize {
    // QModelIndexList_erase_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_erase_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator erase(QList::iterator, QList::iterator)
impl QModelIndexList {
pub fn Erase_1(&self) -> usize {
    // QModelIndexList_erase_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_erase_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// int count()
impl QModelIndexList {
pub fn Count_1(&self) -> i32 {
    // QModelIndexList_count_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_count_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int length()
impl QModelIndexList {
pub fn Length_0(&self) -> i32 {
    // QModelIndexList_length_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_length_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// T & first()
impl QModelIndexList {
pub fn First_0(&self) -> usize {
    // QModelIndexList_first_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_first_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & constFirst()
impl QModelIndexList {
pub fn ConstFirst_0(&self) -> usize {
    // QModelIndexList_constFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_constFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & first()
impl QModelIndexList {
pub fn First_1(&self) -> usize {
    // QModelIndexList_first_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_first_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & last()
impl QModelIndexList {
pub fn Last_0(&self) -> usize {
    // QModelIndexList_last_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_last_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & last()
impl QModelIndexList {
pub fn Last_1(&self) -> usize {
    // QModelIndexList_last_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_last_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & constLast()
impl QModelIndexList {
pub fn ConstLast_0(&self) -> usize {
    // QModelIndexList_constLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_constLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void removeFirst()
impl QModelIndexList {
pub fn RemoveFirst_0(&self) -> (/*void*/) {
    // QModelIndexList_removeFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_removeFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void removeLast()
impl QModelIndexList {
pub fn RemoveLast_0(&self) -> (/*void*/) {
    // QModelIndexList_removeLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_removeLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool startsWith(const T &)
impl QModelIndexList {
pub fn StartsWith_0(&self) -> bool {
    // QModelIndexList_startsWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_startsWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool endsWith(const T &)
impl QModelIndexList {
pub fn EndsWith_0(&self) -> bool {
    // QModelIndexList_endsWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_endsWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QList<T> mid(int, int)
impl QModelIndexList {
pub fn Mid_0(&self) -> usize {
    // QModelIndexList_mid_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_mid_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T value(int)
impl QModelIndexList {
pub fn Value_0(&self) -> usize {
    // QModelIndexList_value_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_value_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T value(int, const T &)
impl QModelIndexList {
pub fn Value_1(&self) -> usize {
    // QModelIndexList_value_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_value_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void push_back(const T &)
impl QModelIndexList {
pub fn Push_back_0(&self) -> (/*void*/) {
    // QModelIndexList_push_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_push_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void push_front(const T &)
impl QModelIndexList {
pub fn Push_front_0(&self) -> (/*void*/) {
    // QModelIndexList_push_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_push_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// T & front()
impl QModelIndexList {
pub fn Front_0(&self) -> usize {
    // QModelIndexList_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & front()
impl QModelIndexList {
pub fn Front_1(&self) -> usize {
    // QModelIndexList_front_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_front_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & back()
impl QModelIndexList {
pub fn Back_0(&self) -> usize {
    // QModelIndexList_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & back()
impl QModelIndexList {
pub fn Back_1(&self) -> usize {
    // QModelIndexList_back_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_back_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void pop_front()
impl QModelIndexList {
pub fn Pop_front_0(&self) -> (/*void*/) {
    // QModelIndexList_pop_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_pop_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void pop_back()
impl QModelIndexList {
pub fn Pop_back_0(&self) -> (/*void*/) {
    // QModelIndexList_pop_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_pop_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool empty()
impl QModelIndexList {
pub fn Empty_0(&self) -> bool {
    // QModelIndexList_empty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_empty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QList<T> & operator+=(const QList<T> &)
impl QModelIndexList {
pub fn Operator_add_equal_0(&self) -> usize {
    // QModelIndexList_operator_add_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_operator_add_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> operator+(const QList<T> &)
impl QModelIndexList {
pub fn Operator_add_0(&self) -> usize {
    // QModelIndexList_operator_add_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_operator_add_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator+=(const T &)
impl QModelIndexList {
pub fn Operator_add_equal_1(&self) -> usize {
    // QModelIndexList_operator_add_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_operator_add_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator<<(const T &)
impl QModelIndexList {
pub fn Operator_left_shift_0(&self) -> usize {
    // QModelIndexList_operator_left_shift_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_operator_left_shift_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator<<(const QList<T> &)
impl QModelIndexList {
pub fn Operator_left_shift_1(&self) -> usize {
    // QModelIndexList_operator_left_shift_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_operator_left_shift_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QVector<T> toVector()
impl QModelIndexList {
pub fn ToVector_0(&self) -> usize {
    // QModelIndexList_toVector_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_toVector_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> toSet()
impl QModelIndexList {
pub fn ToSet_0(&self) -> usize {
    // QModelIndexList_toSet_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_toSet_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromVector(const QVector<T> &)
impl QModelIndexList {
pub fn FromVector_0(&self) -> usize {
    // QModelIndexList_fromVector_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_fromVector_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromSet(const QSet<T> &)
impl QModelIndexList {
pub fn FromSet_0(&self) -> usize {
    // QModelIndexList_fromSet_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_fromSet_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromStdList(const std::list<T> &)
impl QModelIndexList {
pub fn FromStdList_0(&self) -> usize {
    // QModelIndexList_fromStdList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_fromStdList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// std::list<T> toStdList()
impl QModelIndexList {
pub fn ToStdList_0(&self) -> i32 {
    // QModelIndexList_toStdList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_toStdList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :i32 = Default::default();
    return dret;
  }
}
// QList::Node * detach_helper_grow(int, int)
impl QModelIndexList {
pub fn Detach_helper_grow_0(&self) -> usize {
    // QModelIndexList_detach_helper_grow_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_detach_helper_grow_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void detach_helper(int)
impl QModelIndexList {
pub fn Detach_helper_0(&self) -> (/*void*/) {
    // QModelIndexList_detach_helper_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_detach_helper_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detach_helper()
impl QModelIndexList {
pub fn Detach_helper_1(&self) -> (/*void*/) {
    // QModelIndexList_detach_helper_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_detach_helper_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void dealloc(QListData::Data *)
impl QModelIndexList {
pub fn Dealloc_0(&self) -> (/*void*/) {
    // QModelIndexList_dealloc_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_dealloc_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_construct(QList::Node *, const T &)
impl QModelIndexList {
pub fn Node_construct_0(&self) -> (/*void*/) {
    // QModelIndexList_node_construct_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_node_construct_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_destruct(QList::Node *)
impl QModelIndexList {
pub fn Node_destruct_0(&self) -> (/*void*/) {
    // QModelIndexList_node_destruct_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_node_destruct_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_copy(QList::Node *, QList::Node *, QList::Node *)
impl QModelIndexList {
pub fn Node_copy_0(&self) -> (/*void*/) {
    // QModelIndexList_node_copy_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_node_copy_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_destruct(QList::Node *, QList::Node *)
impl QModelIndexList {
pub fn Node_destruct_1(&self) -> (/*void*/) {
    // QModelIndexList_node_destruct_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_node_destruct_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isValidIterator(const QList::iterator &)
impl QModelIndexList {
pub fn IsValidIterator_0(&self) -> bool {
    // QModelIndexList_isValidIterator_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_isValidIterator_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool op_eq_impl(const QList<T> &, QListData::NotArrayCompatibleLayout)
impl QModelIndexList {
pub fn Op_eq_impl_0(&self) -> bool {
    // QModelIndexList_op_eq_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_op_eq_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool op_eq_impl(const QList<T> &, QListData::ArrayCompatibleLayout)
impl QModelIndexList {
pub fn Op_eq_impl_1(&self) -> bool {
    // QModelIndexList_op_eq_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_op_eq_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains_impl(const T &, QListData::NotArrayCompatibleLayout)
impl QModelIndexList {
pub fn Contains_impl_0(&self) -> bool {
    // QModelIndexList_contains_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_contains_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains_impl(const T &, QListData::ArrayCompatibleLayout)
impl QModelIndexList {
pub fn Contains_impl_1(&self) -> bool {
    // QModelIndexList_contains_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_contains_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int count_impl(const T &, QListData::NotArrayCompatibleLayout)
impl QModelIndexList {
pub fn Count_impl_0(&self) -> i32 {
    // QModelIndexList_count_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_count_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int count_impl(const T &, QListData::ArrayCompatibleLayout)
impl QModelIndexList {
pub fn Count_impl_1(&self) -> i32 {
    // QModelIndexList_count_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QModelIndexList_count_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
//  body block end
