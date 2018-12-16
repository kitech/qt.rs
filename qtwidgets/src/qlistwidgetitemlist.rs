
// mod ::widgets::QListWidgetItem
// package qtwidgets
// /usr/include/qt/QtWidgets/qlistwidget.h
// #include <qlistwidget.h>
// #include <QtWidgets>

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
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
// import "github.com/kitech/qt.go/qtgui"
use qtgui::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin
pub struct QListWidgetItemList {
    pub qclsinst: usize /* *mut c_void*/,
}
// QList<T> & operator=(const QList<T> &)
impl QListWidgetItemList {
pub fn Operator_equal_0(&self) -> usize {
    // QListWidgetItemList_operator_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_operator_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator=(QList<T> &&)
impl QListWidgetItemList {
pub fn Operator_equal_1(&self) -> usize {
    // QListWidgetItemList_operator_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_operator_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void swap(QList<T> &)
impl QListWidgetItemList {
pub fn Swap_0(&self) -> (/*void*/) {
    // QListWidgetItemList_swap_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_swap_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool operator==(const QList<T> &)
impl QListWidgetItemList {
pub fn Operator_equal_equal_0(&self) -> bool {
    // QListWidgetItemList_operator_equal_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_operator_equal_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool operator!=(const QList<T> &)
impl QListWidgetItemList {
pub fn Operator_not_equal_0(&self) -> bool {
    // QListWidgetItemList_operator_not_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_operator_not_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int size()
impl QListWidgetItemList {
pub fn Size_0(&self) -> i32 {
    // QListWidgetItemList_size_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_size_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// void detach()
impl QListWidgetItemList {
pub fn Detach_0(&self) -> (/*void*/) {
    // QListWidgetItemList_detach_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_detach_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detachShared()
impl QListWidgetItemList {
pub fn DetachShared_0(&self) -> (/*void*/) {
    // QListWidgetItemList_detachShared_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_detachShared_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isDetached()
impl QListWidgetItemList {
pub fn IsDetached_0(&self) -> bool {
    // QListWidgetItemList_isDetached_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_isDetached_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void setSharable(bool)
impl QListWidgetItemList {
pub fn SetSharable_0(&self) -> (/*void*/) {
    // QListWidgetItemList_setSharable_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_setSharable_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isSharedWith(const QList<T> &)
impl QListWidgetItemList {
pub fn IsSharedWith_0(&self) -> bool {
    // QListWidgetItemList_isSharedWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_isSharedWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool isEmpty()
impl QListWidgetItemList {
pub fn IsEmpty_0(&self) -> bool {
    // QListWidgetItemList_isEmpty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_isEmpty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void clear()
impl QListWidgetItemList {
pub fn Clear_0(&self) -> (/*void*/) {
    // QListWidgetItemList_clear_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_clear_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// const T & at(int)
impl QListWidgetItemList {
pub fn At_0(&self) -> usize {
    // QListWidgetItemList_at_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_at_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & operator[](int)
impl QListWidgetItemList {
pub fn Operator_get_index_0(&self) -> usize {
    // QListWidgetItemList_operator_get_index_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_operator_get_index_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & operator[](int)
impl QListWidgetItemList {
pub fn Operator_get_index_1(&self) -> usize {
    // QListWidgetItemList_operator_get_index_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_operator_get_index_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void reserve(int)
impl QListWidgetItemList {
pub fn Reserve_0(&self) -> (/*void*/) {
    // QListWidgetItemList_reserve_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_reserve_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void append(const T &)
impl QListWidgetItemList {
pub fn Append_0(&self) -> (/*void*/) {
    // QListWidgetItemList_append_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_append_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void append(const QList<T> &)
impl QListWidgetItemList {
pub fn Append_1(&self) -> (/*void*/) {
    // QListWidgetItemList_append_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_append_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void prepend(const T &)
impl QListWidgetItemList {
pub fn Prepend_0(&self) -> (/*void*/) {
    // QListWidgetItemList_prepend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_prepend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void insert(int, const T &)
impl QListWidgetItemList {
pub fn Insert_0(&self) -> (/*void*/) {
    // QListWidgetItemList_insert_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_insert_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void replace(int, const T &)
impl QListWidgetItemList {
pub fn Replace_0(&self) -> (/*void*/) {
    // QListWidgetItemList_replace_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_replace_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void removeAt(int)
impl QListWidgetItemList {
pub fn RemoveAt_0(&self) -> (/*void*/) {
    // QListWidgetItemList_removeAt_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_removeAt_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int removeAll(const T &)
impl QListWidgetItemList {
pub fn RemoveAll_0(&self) -> i32 {
    // QListWidgetItemList_removeAll_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_removeAll_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool removeOne(const T &)
impl QListWidgetItemList {
pub fn RemoveOne_0(&self) -> bool {
    // QListWidgetItemList_removeOne_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_removeOne_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// T takeAt(int)
impl QListWidgetItemList {
pub fn TakeAt_0(&self) -> usize {
    // QListWidgetItemList_takeAt_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_takeAt_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T takeFirst()
impl QListWidgetItemList {
pub fn TakeFirst_0(&self) -> usize {
    // QListWidgetItemList_takeFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_takeFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T takeLast()
impl QListWidgetItemList {
pub fn TakeLast_0(&self) -> usize {
    // QListWidgetItemList_takeLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_takeLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void move(int, int)
impl QListWidgetItemList {
pub fn Move_0(&self) -> (/*void*/) {
    // QListWidgetItemList_move_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_move_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void swap(int, int)
impl QListWidgetItemList {
pub fn Swap_1(&self) -> (/*void*/) {
    // QListWidgetItemList_swap_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_swap_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int indexOf(const T &, int)
impl QListWidgetItemList {
pub fn IndexOf_0(&self) -> i32 {
    // QListWidgetItemList_indexOf_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_indexOf_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int lastIndexOf(const T &, int)
impl QListWidgetItemList {
pub fn LastIndexOf_0(&self) -> i32 {
    // QListWidgetItemList_lastIndexOf_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_lastIndexOf_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool contains(const T &)
impl QListWidgetItemList {
pub fn Contains_0(&self) -> bool {
    // QListWidgetItemList_contains_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_contains_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int count(const T &)
impl QListWidgetItemList {
pub fn Count_0(&self) -> i32 {
    // QListWidgetItemList_count_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_count_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// QList::iterator begin()
impl QListWidgetItemList {
pub fn Begin_0(&self) -> usize {
    // QListWidgetItemList_begin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_begin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator begin()
impl QListWidgetItemList {
pub fn Begin_1(&self) -> usize {
    // QListWidgetItemList_begin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_begin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator cbegin()
impl QListWidgetItemList {
pub fn Cbegin_0(&self) -> usize {
    // QListWidgetItemList_cbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_cbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator constBegin()
impl QListWidgetItemList {
pub fn ConstBegin_0(&self) -> usize {
    // QListWidgetItemList_constBegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_constBegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator end()
impl QListWidgetItemList {
pub fn End_0(&self) -> usize {
    // QListWidgetItemList_end_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_end_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator end()
impl QListWidgetItemList {
pub fn End_1(&self) -> usize {
    // QListWidgetItemList_end_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_end_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator cend()
impl QListWidgetItemList {
pub fn Cend_0(&self) -> usize {
    // QListWidgetItemList_cend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_cend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator constEnd()
impl QListWidgetItemList {
pub fn ConstEnd_0(&self) -> usize {
    // QListWidgetItemList_constEnd_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_constEnd_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::reverse_iterator rbegin()
impl QListWidgetItemList {
pub fn Rbegin_0(&self) -> usize {
    // QListWidgetItemList_rbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_rbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::reverse_iterator rend()
impl QListWidgetItemList {
pub fn Rend_0(&self) -> usize {
    // QListWidgetItemList_rend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_rend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator rbegin()
impl QListWidgetItemList {
pub fn Rbegin_1(&self) -> usize {
    // QListWidgetItemList_rbegin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_rbegin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator rend()
impl QListWidgetItemList {
pub fn Rend_1(&self) -> usize {
    // QListWidgetItemList_rend_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_rend_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator crbegin()
impl QListWidgetItemList {
pub fn Crbegin_0(&self) -> usize {
    // QListWidgetItemList_crbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_crbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator crend()
impl QListWidgetItemList {
pub fn Crend_0(&self) -> usize {
    // QListWidgetItemList_crend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_crend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator insert(QList::iterator, const T &)
impl QListWidgetItemList {
pub fn Insert_1(&self) -> usize {
    // QListWidgetItemList_insert_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_insert_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator erase(QList::iterator)
impl QListWidgetItemList {
pub fn Erase_0(&self) -> usize {
    // QListWidgetItemList_erase_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_erase_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator erase(QList::iterator, QList::iterator)
impl QListWidgetItemList {
pub fn Erase_1(&self) -> usize {
    // QListWidgetItemList_erase_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_erase_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// int count()
impl QListWidgetItemList {
pub fn Count_1(&self) -> i32 {
    // QListWidgetItemList_count_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_count_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int length()
impl QListWidgetItemList {
pub fn Length_0(&self) -> i32 {
    // QListWidgetItemList_length_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_length_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// T & first()
impl QListWidgetItemList {
pub fn First_0(&self) -> usize {
    // QListWidgetItemList_first_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_first_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & constFirst()
impl QListWidgetItemList {
pub fn ConstFirst_0(&self) -> usize {
    // QListWidgetItemList_constFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_constFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & first()
impl QListWidgetItemList {
pub fn First_1(&self) -> usize {
    // QListWidgetItemList_first_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_first_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & last()
impl QListWidgetItemList {
pub fn Last_0(&self) -> usize {
    // QListWidgetItemList_last_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_last_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & last()
impl QListWidgetItemList {
pub fn Last_1(&self) -> usize {
    // QListWidgetItemList_last_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_last_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & constLast()
impl QListWidgetItemList {
pub fn ConstLast_0(&self) -> usize {
    // QListWidgetItemList_constLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_constLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void removeFirst()
impl QListWidgetItemList {
pub fn RemoveFirst_0(&self) -> (/*void*/) {
    // QListWidgetItemList_removeFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_removeFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void removeLast()
impl QListWidgetItemList {
pub fn RemoveLast_0(&self) -> (/*void*/) {
    // QListWidgetItemList_removeLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_removeLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool startsWith(const T &)
impl QListWidgetItemList {
pub fn StartsWith_0(&self) -> bool {
    // QListWidgetItemList_startsWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_startsWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool endsWith(const T &)
impl QListWidgetItemList {
pub fn EndsWith_0(&self) -> bool {
    // QListWidgetItemList_endsWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_endsWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QList<T> mid(int, int)
impl QListWidgetItemList {
pub fn Mid_0(&self) -> usize {
    // QListWidgetItemList_mid_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_mid_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T value(int)
impl QListWidgetItemList {
pub fn Value_0(&self) -> usize {
    // QListWidgetItemList_value_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_value_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T value(int, const T &)
impl QListWidgetItemList {
pub fn Value_1(&self) -> usize {
    // QListWidgetItemList_value_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_value_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void push_back(const T &)
impl QListWidgetItemList {
pub fn Push_back_0(&self) -> (/*void*/) {
    // QListWidgetItemList_push_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_push_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void push_front(const T &)
impl QListWidgetItemList {
pub fn Push_front_0(&self) -> (/*void*/) {
    // QListWidgetItemList_push_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_push_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// T & front()
impl QListWidgetItemList {
pub fn Front_0(&self) -> usize {
    // QListWidgetItemList_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & front()
impl QListWidgetItemList {
pub fn Front_1(&self) -> usize {
    // QListWidgetItemList_front_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_front_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & back()
impl QListWidgetItemList {
pub fn Back_0(&self) -> usize {
    // QListWidgetItemList_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & back()
impl QListWidgetItemList {
pub fn Back_1(&self) -> usize {
    // QListWidgetItemList_back_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_back_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void pop_front()
impl QListWidgetItemList {
pub fn Pop_front_0(&self) -> (/*void*/) {
    // QListWidgetItemList_pop_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_pop_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void pop_back()
impl QListWidgetItemList {
pub fn Pop_back_0(&self) -> (/*void*/) {
    // QListWidgetItemList_pop_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_pop_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool empty()
impl QListWidgetItemList {
pub fn Empty_0(&self) -> bool {
    // QListWidgetItemList_empty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_empty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QList<T> & operator+=(const QList<T> &)
impl QListWidgetItemList {
pub fn Operator_add_equal_0(&self) -> usize {
    // QListWidgetItemList_operator_add_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_operator_add_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> operator+(const QList<T> &)
impl QListWidgetItemList {
pub fn Operator_add_0(&self) -> usize {
    // QListWidgetItemList_operator_add_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_operator_add_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator+=(const T &)
impl QListWidgetItemList {
pub fn Operator_add_equal_1(&self) -> usize {
    // QListWidgetItemList_operator_add_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_operator_add_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator<<(const T &)
impl QListWidgetItemList {
pub fn Operator_left_shift_0(&self) -> usize {
    // QListWidgetItemList_operator_left_shift_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_operator_left_shift_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator<<(const QList<T> &)
impl QListWidgetItemList {
pub fn Operator_left_shift_1(&self) -> usize {
    // QListWidgetItemList_operator_left_shift_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_operator_left_shift_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QVector<T> toVector()
impl QListWidgetItemList {
pub fn ToVector_0(&self) -> usize {
    // QListWidgetItemList_toVector_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_toVector_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> toSet()
impl QListWidgetItemList {
pub fn ToSet_0(&self) -> usize {
    // QListWidgetItemList_toSet_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_toSet_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromVector(const QVector<T> &)
impl QListWidgetItemList {
pub fn FromVector_0(&self) -> usize {
    // QListWidgetItemList_fromVector_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_fromVector_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromSet(const QSet<T> &)
impl QListWidgetItemList {
pub fn FromSet_0(&self) -> usize {
    // QListWidgetItemList_fromSet_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_fromSet_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromStdList(const std::list<T> &)
impl QListWidgetItemList {
pub fn FromStdList_0(&self) -> usize {
    // QListWidgetItemList_fromStdList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_fromStdList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// std::list<T> toStdList()
impl QListWidgetItemList {
pub fn ToStdList_0(&self) -> i32 {
    // QListWidgetItemList_toStdList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_toStdList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :i32 = Default::default();
    return dret;
  }
}
// QList::Node * detach_helper_grow(int, int)
impl QListWidgetItemList {
pub fn Detach_helper_grow_0(&self) -> usize {
    // QListWidgetItemList_detach_helper_grow_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_detach_helper_grow_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void detach_helper(int)
impl QListWidgetItemList {
pub fn Detach_helper_0(&self) -> (/*void*/) {
    // QListWidgetItemList_detach_helper_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_detach_helper_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detach_helper()
impl QListWidgetItemList {
pub fn Detach_helper_1(&self) -> (/*void*/) {
    // QListWidgetItemList_detach_helper_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_detach_helper_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void dealloc(QListData::Data *)
impl QListWidgetItemList {
pub fn Dealloc_0(&self) -> (/*void*/) {
    // QListWidgetItemList_dealloc_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_dealloc_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_construct(QList::Node *, const T &)
impl QListWidgetItemList {
pub fn Node_construct_0(&self) -> (/*void*/) {
    // QListWidgetItemList_node_construct_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_node_construct_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_destruct(QList::Node *)
impl QListWidgetItemList {
pub fn Node_destruct_0(&self) -> (/*void*/) {
    // QListWidgetItemList_node_destruct_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_node_destruct_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_copy(QList::Node *, QList::Node *, QList::Node *)
impl QListWidgetItemList {
pub fn Node_copy_0(&self) -> (/*void*/) {
    // QListWidgetItemList_node_copy_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_node_copy_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_destruct(QList::Node *, QList::Node *)
impl QListWidgetItemList {
pub fn Node_destruct_1(&self) -> (/*void*/) {
    // QListWidgetItemList_node_destruct_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_node_destruct_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isValidIterator(const QList::iterator &)
impl QListWidgetItemList {
pub fn IsValidIterator_0(&self) -> bool {
    // QListWidgetItemList_isValidIterator_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_isValidIterator_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool op_eq_impl(const QList<T> &, QListData::NotArrayCompatibleLayout)
impl QListWidgetItemList {
pub fn Op_eq_impl_0(&self) -> bool {
    // QListWidgetItemList_op_eq_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_op_eq_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool op_eq_impl(const QList<T> &, QListData::ArrayCompatibleLayout)
impl QListWidgetItemList {
pub fn Op_eq_impl_1(&self) -> bool {
    // QListWidgetItemList_op_eq_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_op_eq_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains_impl(const T &, QListData::NotArrayCompatibleLayout)
impl QListWidgetItemList {
pub fn Contains_impl_0(&self) -> bool {
    // QListWidgetItemList_contains_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_contains_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains_impl(const T &, QListData::ArrayCompatibleLayout)
impl QListWidgetItemList {
pub fn Contains_impl_1(&self) -> bool {
    // QListWidgetItemList_contains_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_contains_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int count_impl(const T &, QListData::NotArrayCompatibleLayout)
impl QListWidgetItemList {
pub fn Count_impl_0(&self) -> i32 {
    // QListWidgetItemList_count_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_count_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int count_impl(const T &, QListData::ArrayCompatibleLayout)
impl QListWidgetItemList {
pub fn Count_impl_1(&self) -> i32 {
    // QListWidgetItemList_count_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QListWidgetItemList_count_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
//  body block end
