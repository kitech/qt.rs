
// mod ::core::QString
// package qtcore
// /usr/include/qt/QtCore/qstring.h
// #include <qstring.h>
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
pub struct QVariantHash {
    pub qclsinst: usize /* *mut c_void*/,
}
// QHash::Node * concrete(QHashData::Node *)
impl QVariantHash {
pub fn Concrete_0(&self) -> usize {
    // QVariantHash_concrete_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_concrete_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// int alignOfNode()
impl QVariantHash {
pub fn AlignOfNode_0(&self) -> i32 {
    // QVariantHash_alignOfNode_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_alignOfNode_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// QHash<K, V> & operator=(const QHash<K, V> &)
impl QVariantHash {
pub fn Operator_equal_0(&self) -> usize {
    // QVariantHash_operator_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_operator_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash<K, V> & operator=(QHash<K, V> &&)
impl QVariantHash {
pub fn Operator_equal_1(&self) -> usize {
    // QVariantHash_operator_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_operator_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void swap(QHash<K, V> &)
impl QVariantHash {
pub fn Swap_0(&self) -> (/*void*/) {
    // QVariantHash_swap_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_swap_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool operator==(const QHash<K, V> &)
impl QVariantHash {
pub fn Operator_equal_equal_0(&self) -> bool {
    // QVariantHash_operator_equal_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_operator_equal_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool operator!=(const QHash<K, V> &)
impl QVariantHash {
pub fn Operator_not_equal_0(&self) -> bool {
    // QVariantHash_operator_not_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_operator_not_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int size()
impl QVariantHash {
pub fn Size_0(&self) -> i32 {
    // QVariantHash_size_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_size_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool isEmpty()
impl QVariantHash {
pub fn IsEmpty_0(&self) -> bool {
    // QVariantHash_isEmpty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_isEmpty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int capacity()
impl QVariantHash {
pub fn Capacity_0(&self) -> i32 {
    // QVariantHash_capacity_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_capacity_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// void reserve(int)
impl QVariantHash {
pub fn Reserve_0(&self) -> (/*void*/) {
    // QVariantHash_reserve_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_reserve_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void squeeze()
impl QVariantHash {
pub fn Squeeze_0(&self) -> (/*void*/) {
    // QVariantHash_squeeze_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_squeeze_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detach()
impl QVariantHash {
pub fn Detach_0(&self) -> (/*void*/) {
    // QVariantHash_detach_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_detach_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isDetached()
impl QVariantHash {
pub fn IsDetached_0(&self) -> bool {
    // QVariantHash_isDetached_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_isDetached_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void setSharable(bool)
impl QVariantHash {
pub fn SetSharable_0(&self) -> (/*void*/) {
    // QVariantHash_setSharable_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_setSharable_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isSharedWith(const QHash<K, V> &)
impl QVariantHash {
pub fn IsSharedWith_0(&self) -> bool {
    // QVariantHash_isSharedWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_isSharedWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void clear()
impl QVariantHash {
pub fn Clear_0(&self) -> (/*void*/) {
    // QVariantHash_clear_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_clear_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int remove(const Key &)
impl QVariantHash {
pub fn Remove_0(&self) -> i32 {
    // QVariantHash_remove_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_remove_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// T take(const Key &)
impl QVariantHash {
pub fn Take_0(&self) -> usize {
    // QVariantHash_take_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_take_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// bool contains(const Key &)
impl QVariantHash {
pub fn Contains_0(&self) -> bool {
    // QVariantHash_contains_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_contains_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// const Key key(const T &)
impl QVariantHash {
pub fn Key_0(&self) -> usize {
    // QVariantHash_key_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_key_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const Key key(const T &, const Key &)
impl QVariantHash {
pub fn Key_1(&self) -> usize {
    // QVariantHash_key_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_key_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T value(const Key &)
impl QVariantHash {
pub fn Value_0(&self) -> usize {
    // QVariantHash_value_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_value_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T value(const Key &, const T &)
impl QVariantHash {
pub fn Value_1(&self) -> usize {
    // QVariantHash_value_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_value_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & operator[](const Key &)
impl QVariantHash {
pub fn Operator_get_index_0(&self) -> usize {
    // QVariantHash_operator_get_index_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_operator_get_index_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T operator[](const Key &)
impl QVariantHash {
pub fn Operator_get_index_1(&self) -> usize {
    // QVariantHash_operator_get_index_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_operator_get_index_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<Key> uniqueKeys()
impl QVariantHash {
pub fn UniqueKeys_0(&self) -> usize {
    // QVariantHash_uniqueKeys_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_uniqueKeys_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<Key> keys()
impl QVariantHash {
pub fn Keys_0(&self) -> usize {
    // QVariantHash_keys_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_keys_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<Key> keys(const T &)
impl QVariantHash {
pub fn Keys_1(&self) -> usize {
    // QVariantHash_keys_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_keys_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> values()
impl QVariantHash {
pub fn Values_0(&self) -> usize {
    // QVariantHash_values_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_values_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> values(const Key &)
impl QVariantHash {
pub fn Values_1(&self) -> usize {
    // QVariantHash_values_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_values_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// int count(const Key &)
impl QVariantHash {
pub fn Count_0(&self) -> i32 {
    // QVariantHash_count_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_count_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// QHash::iterator begin()
impl QVariantHash {
pub fn Begin_0(&self) -> usize {
    // QVariantHash_begin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_begin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::const_iterator begin()
impl QVariantHash {
pub fn Begin_1(&self) -> usize {
    // QVariantHash_begin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_begin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::const_iterator cbegin()
impl QVariantHash {
pub fn Cbegin_0(&self) -> usize {
    // QVariantHash_cbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_cbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::const_iterator constBegin()
impl QVariantHash {
pub fn ConstBegin_0(&self) -> usize {
    // QVariantHash_constBegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_constBegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::iterator end()
impl QVariantHash {
pub fn End_0(&self) -> usize {
    // QVariantHash_end_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_end_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::const_iterator end()
impl QVariantHash {
pub fn End_1(&self) -> usize {
    // QVariantHash_end_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_end_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::const_iterator cend()
impl QVariantHash {
pub fn Cend_0(&self) -> usize {
    // QVariantHash_cend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_cend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::const_iterator constEnd()
impl QVariantHash {
pub fn ConstEnd_0(&self) -> usize {
    // QVariantHash_constEnd_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_constEnd_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::key_iterator keyBegin()
impl QVariantHash {
pub fn KeyBegin_0(&self) -> usize {
    // QVariantHash_keyBegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_keyBegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::key_iterator keyEnd()
impl QVariantHash {
pub fn KeyEnd_0(&self) -> usize {
    // QVariantHash_keyEnd_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_keyEnd_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::key_value_iterator keyValueBegin()
impl QVariantHash {
pub fn KeyValueBegin_0(&self) -> usize {
    // QVariantHash_keyValueBegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_keyValueBegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::key_value_iterator keyValueEnd()
impl QVariantHash {
pub fn KeyValueEnd_0(&self) -> usize {
    // QVariantHash_keyValueEnd_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_keyValueEnd_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::const_key_value_iterator keyValueBegin()
impl QVariantHash {
pub fn KeyValueBegin_1(&self) -> usize {
    // QVariantHash_keyValueBegin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_keyValueBegin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::const_key_value_iterator constKeyValueBegin()
impl QVariantHash {
pub fn ConstKeyValueBegin_0(&self) -> usize {
    // QVariantHash_constKeyValueBegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_constKeyValueBegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::const_key_value_iterator keyValueEnd()
impl QVariantHash {
pub fn KeyValueEnd_1(&self) -> usize {
    // QVariantHash_keyValueEnd_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_keyValueEnd_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::const_key_value_iterator constKeyValueEnd()
impl QVariantHash {
pub fn ConstKeyValueEnd_0(&self) -> usize {
    // QVariantHash_constKeyValueEnd_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_constKeyValueEnd_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QPair<QHash::iterator, QHash::iterator> equal_range(const Key &)
impl QVariantHash {
pub fn Equal_range_0(&self) -> usize {
    // QVariantHash_equal_range_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_equal_range_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QPair<QHash::const_iterator, QHash::const_iterator> equal_range(const Key &)
impl QVariantHash {
pub fn Equal_range_1(&self) -> usize {
    // QVariantHash_equal_range_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_equal_range_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::iterator erase(QHash::iterator)
impl QVariantHash {
pub fn Erase_0(&self) -> usize {
    // QVariantHash_erase_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_erase_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::iterator erase(QHash::const_iterator)
impl QVariantHash {
pub fn Erase_1(&self) -> usize {
    // QVariantHash_erase_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_erase_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// int count()
impl QVariantHash {
pub fn Count_1(&self) -> i32 {
    // QVariantHash_count_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_count_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// QHash::iterator find(const Key &)
impl QVariantHash {
pub fn Find_0(&self) -> usize {
    // QVariantHash_find_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_find_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::const_iterator find(const Key &)
impl QVariantHash {
pub fn Find_1(&self) -> usize {
    // QVariantHash_find_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_find_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::const_iterator constFind(const Key &)
impl QVariantHash {
pub fn ConstFind_0(&self) -> usize {
    // QVariantHash_constFind_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_constFind_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::iterator insert(const Key &, const T &)
impl QVariantHash {
pub fn Insert_0(&self) -> usize {
    // QVariantHash_insert_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_insert_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::iterator insertMulti(const Key &, const T &)
impl QVariantHash {
pub fn InsertMulti_0(&self) -> usize {
    // QVariantHash_insertMulti_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_insertMulti_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash<K, V> & unite(const QHash<K, V> &)
impl QVariantHash {
pub fn Unite_0(&self) -> usize {
    // QVariantHash_unite_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_unite_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// bool empty()
impl QVariantHash {
pub fn Empty_0(&self) -> bool {
    // QVariantHash_empty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_empty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void detach_helper()
impl QVariantHash {
pub fn Detach_helper_0(&self) -> (/*void*/) {
    // QVariantHash_detach_helper_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_detach_helper_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void freeData(QHashData *)
impl QVariantHash {
pub fn FreeData_0(&self) -> (/*void*/) {
    // QVariantHash_freeData_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_freeData_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// QHash::Node ** findNode(const Key &, uint *)
impl QVariantHash {
pub fn FindNode_0(&self) -> usize {
    // QVariantHash_findNode_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_findNode_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::Node ** findNode(const Key &, uint)
impl QVariantHash {
pub fn FindNode_1(&self) -> usize {
    // QVariantHash_findNode_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_findNode_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QHash::Node * createNode(uint, const Key &, const T &, QHash::Node **)
impl QVariantHash {
pub fn CreateNode_0(&self) -> usize {
    // QVariantHash_createNode_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_createNode_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void deleteNode(QHash::Node *)
impl QVariantHash {
pub fn DeleteNode_0(&self) -> (/*void*/) {
    // QVariantHash_deleteNode_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_deleteNode_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void deleteNode2(QHashData::Node *)
impl QVariantHash {
pub fn DeleteNode2_0(&self) -> (/*void*/) {
    // QVariantHash_deleteNode2_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_deleteNode2_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void duplicateNode(QHashData::Node *, void *)
impl QVariantHash {
pub fn DuplicateNode_0(&self) -> (/*void*/) {
    // QVariantHash_duplicateNode_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_duplicateNode_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isValidIterator(const QHash::iterator &)
impl QVariantHash {
pub fn IsValidIterator_0(&self) -> bool {
    // QVariantHash_isValidIterator_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_isValidIterator_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool isValidIterator(const QHash::const_iterator &)
impl QVariantHash {
pub fn IsValidIterator_1(&self) -> bool {
    // QVariantHash_isValidIterator_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_isValidIterator_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool isValidNode(QHashData::Node *)
impl QVariantHash {
pub fn IsValidNode_0(&self) -> bool {
    // QVariantHash_isValidNode_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantHash_isValidNode_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
//  body block end
