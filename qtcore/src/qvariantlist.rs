
// mod ::core::QVariant
// package qtcore
// /usr/include/qt/QtCore/qvariant.h
// #include <qvariant.h>
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
pub struct QVariantList {
    pub qclsinst: usize /* *mut c_void*/,
}
// QList<T> & operator=(const QList<T> &)
impl QVariantList {
pub fn Operator_equal_0(&self) -> usize {
    // QVariantList_operator_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_operator_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator=(QList<T> &&)
impl QVariantList {
pub fn Operator_equal_1(&self) -> usize {
    // QVariantList_operator_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_operator_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void swap(QList<T> &)
impl QVariantList {
pub fn Swap_0(&self) -> (/*void*/) {
    // QVariantList_swap_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_swap_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool operator==(const QList<T> &)
impl QVariantList {
pub fn Operator_equal_equal_0(&self) -> bool {
    // QVariantList_operator_equal_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_operator_equal_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool operator!=(const QList<T> &)
impl QVariantList {
pub fn Operator_not_equal_0(&self) -> bool {
    // QVariantList_operator_not_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_operator_not_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int size()
impl QVariantList {
pub fn Size_0(&self) -> i32 {
    // QVariantList_size_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_size_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// void detach()
impl QVariantList {
pub fn Detach_0(&self) -> (/*void*/) {
    // QVariantList_detach_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_detach_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detachShared()
impl QVariantList {
pub fn DetachShared_0(&self) -> (/*void*/) {
    // QVariantList_detachShared_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_detachShared_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isDetached()
impl QVariantList {
pub fn IsDetached_0(&self) -> bool {
    // QVariantList_isDetached_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_isDetached_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void setSharable(bool)
impl QVariantList {
pub fn SetSharable_0(&self) -> (/*void*/) {
    // QVariantList_setSharable_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_setSharable_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isSharedWith(const QList<T> &)
impl QVariantList {
pub fn IsSharedWith_0(&self) -> bool {
    // QVariantList_isSharedWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_isSharedWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool isEmpty()
impl QVariantList {
pub fn IsEmpty_0(&self) -> bool {
    // QVariantList_isEmpty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_isEmpty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void clear()
impl QVariantList {
pub fn Clear_0(&self) -> (/*void*/) {
    // QVariantList_clear_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_clear_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// const T & at(int)
impl QVariantList {
pub fn At_0(&self) -> usize {
    // QVariantList_at_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_at_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & operator[](int)
impl QVariantList {
pub fn Operator_get_index_0(&self) -> usize {
    // QVariantList_operator_get_index_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_operator_get_index_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & operator[](int)
impl QVariantList {
pub fn Operator_get_index_1(&self) -> usize {
    // QVariantList_operator_get_index_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_operator_get_index_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void reserve(int)
impl QVariantList {
pub fn Reserve_0(&self) -> (/*void*/) {
    // QVariantList_reserve_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_reserve_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void append(const T &)
impl QVariantList {
pub fn Append_0(&self) -> (/*void*/) {
    // QVariantList_append_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_append_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void append(const QList<T> &)
impl QVariantList {
pub fn Append_1(&self) -> (/*void*/) {
    // QVariantList_append_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_append_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void prepend(const T &)
impl QVariantList {
pub fn Prepend_0(&self) -> (/*void*/) {
    // QVariantList_prepend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_prepend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void insert(int, const T &)
impl QVariantList {
pub fn Insert_0(&self) -> (/*void*/) {
    // QVariantList_insert_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_insert_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void replace(int, const T &)
impl QVariantList {
pub fn Replace_0(&self) -> (/*void*/) {
    // QVariantList_replace_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_replace_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void removeAt(int)
impl QVariantList {
pub fn RemoveAt_0(&self) -> (/*void*/) {
    // QVariantList_removeAt_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_removeAt_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int removeAll(const T &)
impl QVariantList {
pub fn RemoveAll_0(&self) -> i32 {
    // QVariantList_removeAll_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_removeAll_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool removeOne(const T &)
impl QVariantList {
pub fn RemoveOne_0(&self) -> bool {
    // QVariantList_removeOne_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_removeOne_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// T takeAt(int)
impl QVariantList {
pub fn TakeAt_0(&self) -> usize {
    // QVariantList_takeAt_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_takeAt_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T takeFirst()
impl QVariantList {
pub fn TakeFirst_0(&self) -> usize {
    // QVariantList_takeFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_takeFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T takeLast()
impl QVariantList {
pub fn TakeLast_0(&self) -> usize {
    // QVariantList_takeLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_takeLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void move(int, int)
impl QVariantList {
pub fn Move_0(&self) -> (/*void*/) {
    // QVariantList_move_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_move_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void swap(int, int)
impl QVariantList {
pub fn Swap_1(&self) -> (/*void*/) {
    // QVariantList_swap_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_swap_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int indexOf(const T &, int)
impl QVariantList {
pub fn IndexOf_0(&self) -> i32 {
    // QVariantList_indexOf_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_indexOf_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int lastIndexOf(const T &, int)
impl QVariantList {
pub fn LastIndexOf_0(&self) -> i32 {
    // QVariantList_lastIndexOf_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_lastIndexOf_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool contains(const T &)
impl QVariantList {
pub fn Contains_0(&self) -> bool {
    // QVariantList_contains_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_contains_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int count(const T &)
impl QVariantList {
pub fn Count_0(&self) -> i32 {
    // QVariantList_count_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_count_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// QList::iterator begin()
impl QVariantList {
pub fn Begin_0(&self) -> usize {
    // QVariantList_begin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_begin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator begin()
impl QVariantList {
pub fn Begin_1(&self) -> usize {
    // QVariantList_begin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_begin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator cbegin()
impl QVariantList {
pub fn Cbegin_0(&self) -> usize {
    // QVariantList_cbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_cbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator constBegin()
impl QVariantList {
pub fn ConstBegin_0(&self) -> usize {
    // QVariantList_constBegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_constBegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator end()
impl QVariantList {
pub fn End_0(&self) -> usize {
    // QVariantList_end_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_end_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator end()
impl QVariantList {
pub fn End_1(&self) -> usize {
    // QVariantList_end_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_end_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator cend()
impl QVariantList {
pub fn Cend_0(&self) -> usize {
    // QVariantList_cend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_cend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator constEnd()
impl QVariantList {
pub fn ConstEnd_0(&self) -> usize {
    // QVariantList_constEnd_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_constEnd_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::reverse_iterator rbegin()
impl QVariantList {
pub fn Rbegin_0(&self) -> usize {
    // QVariantList_rbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_rbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::reverse_iterator rend()
impl QVariantList {
pub fn Rend_0(&self) -> usize {
    // QVariantList_rend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_rend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator rbegin()
impl QVariantList {
pub fn Rbegin_1(&self) -> usize {
    // QVariantList_rbegin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_rbegin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator rend()
impl QVariantList {
pub fn Rend_1(&self) -> usize {
    // QVariantList_rend_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_rend_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator crbegin()
impl QVariantList {
pub fn Crbegin_0(&self) -> usize {
    // QVariantList_crbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_crbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator crend()
impl QVariantList {
pub fn Crend_0(&self) -> usize {
    // QVariantList_crend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_crend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator insert(QList::iterator, const T &)
impl QVariantList {
pub fn Insert_1(&self) -> usize {
    // QVariantList_insert_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_insert_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator erase(QList::iterator)
impl QVariantList {
pub fn Erase_0(&self) -> usize {
    // QVariantList_erase_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_erase_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator erase(QList::iterator, QList::iterator)
impl QVariantList {
pub fn Erase_1(&self) -> usize {
    // QVariantList_erase_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_erase_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// int count()
impl QVariantList {
pub fn Count_1(&self) -> i32 {
    // QVariantList_count_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_count_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int length()
impl QVariantList {
pub fn Length_0(&self) -> i32 {
    // QVariantList_length_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_length_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// T & first()
impl QVariantList {
pub fn First_0(&self) -> usize {
    // QVariantList_first_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_first_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & constFirst()
impl QVariantList {
pub fn ConstFirst_0(&self) -> usize {
    // QVariantList_constFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_constFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & first()
impl QVariantList {
pub fn First_1(&self) -> usize {
    // QVariantList_first_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_first_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & last()
impl QVariantList {
pub fn Last_0(&self) -> usize {
    // QVariantList_last_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_last_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & last()
impl QVariantList {
pub fn Last_1(&self) -> usize {
    // QVariantList_last_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_last_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & constLast()
impl QVariantList {
pub fn ConstLast_0(&self) -> usize {
    // QVariantList_constLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_constLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void removeFirst()
impl QVariantList {
pub fn RemoveFirst_0(&self) -> (/*void*/) {
    // QVariantList_removeFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_removeFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void removeLast()
impl QVariantList {
pub fn RemoveLast_0(&self) -> (/*void*/) {
    // QVariantList_removeLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_removeLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool startsWith(const T &)
impl QVariantList {
pub fn StartsWith_0(&self) -> bool {
    // QVariantList_startsWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_startsWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool endsWith(const T &)
impl QVariantList {
pub fn EndsWith_0(&self) -> bool {
    // QVariantList_endsWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_endsWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QList<T> mid(int, int)
impl QVariantList {
pub fn Mid_0(&self) -> usize {
    // QVariantList_mid_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_mid_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T value(int)
impl QVariantList {
pub fn Value_0(&self) -> usize {
    // QVariantList_value_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_value_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T value(int, const T &)
impl QVariantList {
pub fn Value_1(&self) -> usize {
    // QVariantList_value_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_value_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void push_back(const T &)
impl QVariantList {
pub fn Push_back_0(&self) -> (/*void*/) {
    // QVariantList_push_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_push_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void push_front(const T &)
impl QVariantList {
pub fn Push_front_0(&self) -> (/*void*/) {
    // QVariantList_push_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_push_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// T & front()
impl QVariantList {
pub fn Front_0(&self) -> usize {
    // QVariantList_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & front()
impl QVariantList {
pub fn Front_1(&self) -> usize {
    // QVariantList_front_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_front_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & back()
impl QVariantList {
pub fn Back_0(&self) -> usize {
    // QVariantList_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & back()
impl QVariantList {
pub fn Back_1(&self) -> usize {
    // QVariantList_back_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_back_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void pop_front()
impl QVariantList {
pub fn Pop_front_0(&self) -> (/*void*/) {
    // QVariantList_pop_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_pop_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void pop_back()
impl QVariantList {
pub fn Pop_back_0(&self) -> (/*void*/) {
    // QVariantList_pop_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_pop_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool empty()
impl QVariantList {
pub fn Empty_0(&self) -> bool {
    // QVariantList_empty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_empty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QList<T> & operator+=(const QList<T> &)
impl QVariantList {
pub fn Operator_add_equal_0(&self) -> usize {
    // QVariantList_operator_add_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_operator_add_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> operator+(const QList<T> &)
impl QVariantList {
pub fn Operator_add_0(&self) -> usize {
    // QVariantList_operator_add_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_operator_add_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator+=(const T &)
impl QVariantList {
pub fn Operator_add_equal_1(&self) -> usize {
    // QVariantList_operator_add_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_operator_add_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator<<(const T &)
impl QVariantList {
pub fn Operator_left_shift_0(&self) -> usize {
    // QVariantList_operator_left_shift_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_operator_left_shift_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator<<(const QList<T> &)
impl QVariantList {
pub fn Operator_left_shift_1(&self) -> usize {
    // QVariantList_operator_left_shift_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_operator_left_shift_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QVector<T> toVector()
impl QVariantList {
pub fn ToVector_0(&self) -> usize {
    // QVariantList_toVector_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_toVector_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> toSet()
impl QVariantList {
pub fn ToSet_0(&self) -> usize {
    // QVariantList_toSet_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_toSet_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromVector(const QVector<T> &)
impl QVariantList {
pub fn FromVector_0(&self) -> usize {
    // QVariantList_fromVector_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_fromVector_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromSet(const QSet<T> &)
impl QVariantList {
pub fn FromSet_0(&self) -> usize {
    // QVariantList_fromSet_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_fromSet_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromStdList(const std::list<T> &)
impl QVariantList {
pub fn FromStdList_0(&self) -> usize {
    // QVariantList_fromStdList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_fromStdList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// std::list<T> toStdList()
impl QVariantList {
pub fn ToStdList_0(&self) -> i32 {
    // QVariantList_toStdList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_toStdList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :i32 = Default::default();
    return dret;
  }
}
// QList::Node * detach_helper_grow(int, int)
impl QVariantList {
pub fn Detach_helper_grow_0(&self) -> usize {
    // QVariantList_detach_helper_grow_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_detach_helper_grow_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void detach_helper(int)
impl QVariantList {
pub fn Detach_helper_0(&self) -> (/*void*/) {
    // QVariantList_detach_helper_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_detach_helper_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detach_helper()
impl QVariantList {
pub fn Detach_helper_1(&self) -> (/*void*/) {
    // QVariantList_detach_helper_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_detach_helper_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void dealloc(QListData::Data *)
impl QVariantList {
pub fn Dealloc_0(&self) -> (/*void*/) {
    // QVariantList_dealloc_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_dealloc_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_construct(QList::Node *, const T &)
impl QVariantList {
pub fn Node_construct_0(&self) -> (/*void*/) {
    // QVariantList_node_construct_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_node_construct_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_destruct(QList::Node *)
impl QVariantList {
pub fn Node_destruct_0(&self) -> (/*void*/) {
    // QVariantList_node_destruct_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_node_destruct_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_copy(QList::Node *, QList::Node *, QList::Node *)
impl QVariantList {
pub fn Node_copy_0(&self) -> (/*void*/) {
    // QVariantList_node_copy_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_node_copy_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_destruct(QList::Node *, QList::Node *)
impl QVariantList {
pub fn Node_destruct_1(&self) -> (/*void*/) {
    // QVariantList_node_destruct_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_node_destruct_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isValidIterator(const QList::iterator &)
impl QVariantList {
pub fn IsValidIterator_0(&self) -> bool {
    // QVariantList_isValidIterator_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_isValidIterator_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool op_eq_impl(const QList<T> &, QListData::NotArrayCompatibleLayout)
impl QVariantList {
pub fn Op_eq_impl_0(&self) -> bool {
    // QVariantList_op_eq_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_op_eq_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool op_eq_impl(const QList<T> &, QListData::ArrayCompatibleLayout)
impl QVariantList {
pub fn Op_eq_impl_1(&self) -> bool {
    // QVariantList_op_eq_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_op_eq_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains_impl(const T &, QListData::NotArrayCompatibleLayout)
impl QVariantList {
pub fn Contains_impl_0(&self) -> bool {
    // QVariantList_contains_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_contains_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains_impl(const T &, QListData::ArrayCompatibleLayout)
impl QVariantList {
pub fn Contains_impl_1(&self) -> bool {
    // QVariantList_contains_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_contains_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int count_impl(const T &, QListData::NotArrayCompatibleLayout)
impl QVariantList {
pub fn Count_impl_0(&self) -> i32 {
    // QVariantList_count_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_count_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int count_impl(const T &, QListData::ArrayCompatibleLayout)
impl QVariantList {
pub fn Count_impl_1(&self) -> i32 {
    // QVariantList_count_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantList_count_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
//  body block end
