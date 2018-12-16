
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
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin
pub struct QVariantMap {
    pub qclsinst: usize /* *mut c_void*/,
}
// QMap<Key, T> & operator=(const QMap<Key, T> &)
impl QVariantMap {
pub fn Operator_equal_0(&self) -> usize {
    // QVariantMap_operator_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_operator_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap<Key, T> & operator=(QMap<Key, T> &&)
impl QVariantMap {
pub fn Operator_equal_1(&self) -> usize {
    // QVariantMap_operator_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_operator_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void swap(QMap<Key, T> &)
impl QVariantMap {
pub fn Swap_0(&self) -> (/*void*/) {
    // QVariantMap_swap_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_swap_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// std::map<Key, T> toStdMap()
impl QVariantMap {
pub fn ToStdMap_0(&self) -> i32 {
    // QVariantMap_toStdMap_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_toStdMap_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :i32 = Default::default();
    return dret;
  }
}
// bool operator==(const QMap<Key, T> &)
impl QVariantMap {
pub fn Operator_equal_equal_0(&self) -> bool {
    // QVariantMap_operator_equal_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_operator_equal_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool operator!=(const QMap<Key, T> &)
impl QVariantMap {
pub fn Operator_not_equal_0(&self) -> bool {
    // QVariantMap_operator_not_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_operator_not_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int size()
impl QVariantMap {
pub fn Size_0(&self) -> i32 {
    // QVariantMap_size_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_size_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool isEmpty()
impl QVariantMap {
pub fn IsEmpty_0(&self) -> bool {
    // QVariantMap_isEmpty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_isEmpty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void detach()
impl QVariantMap {
pub fn Detach_0(&self) -> (/*void*/) {
    // QVariantMap_detach_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_detach_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isDetached()
impl QVariantMap {
pub fn IsDetached_0(&self) -> bool {
    // QVariantMap_isDetached_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_isDetached_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void setSharable(bool)
impl QVariantMap {
pub fn SetSharable_0(&self) -> (/*void*/) {
    // QVariantMap_setSharable_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_setSharable_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isSharedWith(const QMap<Key, T> &)
impl QVariantMap {
pub fn IsSharedWith_0(&self) -> bool {
    // QVariantMap_isSharedWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_isSharedWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void clear()
impl QVariantMap {
pub fn Clear_0(&self) -> (/*void*/) {
    // QVariantMap_clear_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_clear_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int remove(const Key &)
impl QVariantMap {
pub fn Remove_0(&self) -> i32 {
    // QVariantMap_remove_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_remove_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// T take(const Key &)
impl QVariantMap {
pub fn Take_0(&self) -> usize {
    // QVariantMap_take_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_take_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// bool contains(const Key &)
impl QVariantMap {
pub fn Contains_0(&self) -> bool {
    // QVariantMap_contains_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_contains_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// const Key key(const T &, const Key &)
impl QVariantMap {
pub fn Key_0(&self) -> usize {
    // QVariantMap_key_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_key_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T value(const Key &, const T &)
impl QVariantMap {
pub fn Value_0(&self) -> usize {
    // QVariantMap_value_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_value_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & operator[](const Key &)
impl QVariantMap {
pub fn Operator_get_index_0(&self) -> usize {
    // QVariantMap_operator_get_index_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_operator_get_index_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T operator[](const Key &)
impl QVariantMap {
pub fn Operator_get_index_1(&self) -> usize {
    // QVariantMap_operator_get_index_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_operator_get_index_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<Key> uniqueKeys()
impl QVariantMap {
pub fn UniqueKeys_0(&self) -> usize {
    // QVariantMap_uniqueKeys_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_uniqueKeys_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<Key> keys()
impl QVariantMap {
pub fn Keys_0(&self) -> usize {
    // QVariantMap_keys_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_keys_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<Key> keys(const T &)
impl QVariantMap {
pub fn Keys_1(&self) -> usize {
    // QVariantMap_keys_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_keys_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> values()
impl QVariantMap {
pub fn Values_0(&self) -> usize {
    // QVariantMap_values_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_values_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> values(const Key &)
impl QVariantMap {
pub fn Values_1(&self) -> usize {
    // QVariantMap_values_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_values_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// int count(const Key &)
impl QVariantMap {
pub fn Count_0(&self) -> i32 {
    // QVariantMap_count_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_count_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// const Key & firstKey()
impl QVariantMap {
pub fn FirstKey_0(&self) -> usize {
    // QVariantMap_firstKey_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_firstKey_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const Key & lastKey()
impl QVariantMap {
pub fn LastKey_0(&self) -> usize {
    // QVariantMap_lastKey_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_lastKey_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & first()
impl QVariantMap {
pub fn First_0(&self) -> usize {
    // QVariantMap_first_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_first_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & first()
impl QVariantMap {
pub fn First_1(&self) -> usize {
    // QVariantMap_first_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_first_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & last()
impl QVariantMap {
pub fn Last_0(&self) -> usize {
    // QVariantMap_last_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_last_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & last()
impl QVariantMap {
pub fn Last_1(&self) -> usize {
    // QVariantMap_last_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_last_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::iterator begin()
impl QVariantMap {
pub fn Begin_0(&self) -> usize {
    // QVariantMap_begin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_begin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::const_iterator begin()
impl QVariantMap {
pub fn Begin_1(&self) -> usize {
    // QVariantMap_begin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_begin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::const_iterator constBegin()
impl QVariantMap {
pub fn ConstBegin_0(&self) -> usize {
    // QVariantMap_constBegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_constBegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::const_iterator cbegin()
impl QVariantMap {
pub fn Cbegin_0(&self) -> usize {
    // QVariantMap_cbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_cbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::iterator end()
impl QVariantMap {
pub fn End_0(&self) -> usize {
    // QVariantMap_end_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_end_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::const_iterator end()
impl QVariantMap {
pub fn End_1(&self) -> usize {
    // QVariantMap_end_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_end_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::const_iterator constEnd()
impl QVariantMap {
pub fn ConstEnd_0(&self) -> usize {
    // QVariantMap_constEnd_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_constEnd_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::const_iterator cend()
impl QVariantMap {
pub fn Cend_0(&self) -> usize {
    // QVariantMap_cend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_cend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::key_iterator keyBegin()
impl QVariantMap {
pub fn KeyBegin_0(&self) -> usize {
    // QVariantMap_keyBegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_keyBegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::key_iterator keyEnd()
impl QVariantMap {
pub fn KeyEnd_0(&self) -> usize {
    // QVariantMap_keyEnd_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_keyEnd_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::key_value_iterator keyValueBegin()
impl QVariantMap {
pub fn KeyValueBegin_0(&self) -> usize {
    // QVariantMap_keyValueBegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_keyValueBegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::key_value_iterator keyValueEnd()
impl QVariantMap {
pub fn KeyValueEnd_0(&self) -> usize {
    // QVariantMap_keyValueEnd_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_keyValueEnd_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::const_key_value_iterator keyValueBegin()
impl QVariantMap {
pub fn KeyValueBegin_1(&self) -> usize {
    // QVariantMap_keyValueBegin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_keyValueBegin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::const_key_value_iterator constKeyValueBegin()
impl QVariantMap {
pub fn ConstKeyValueBegin_0(&self) -> usize {
    // QVariantMap_constKeyValueBegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_constKeyValueBegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::const_key_value_iterator keyValueEnd()
impl QVariantMap {
pub fn KeyValueEnd_1(&self) -> usize {
    // QVariantMap_keyValueEnd_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_keyValueEnd_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::const_key_value_iterator constKeyValueEnd()
impl QVariantMap {
pub fn ConstKeyValueEnd_0(&self) -> usize {
    // QVariantMap_constKeyValueEnd_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_constKeyValueEnd_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::iterator erase(QMap::iterator)
impl QVariantMap {
pub fn Erase_0(&self) -> usize {
    // QVariantMap_erase_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_erase_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// int count()
impl QVariantMap {
pub fn Count_1(&self) -> i32 {
    // QVariantMap_count_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_count_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// QMap::iterator find(const Key &)
impl QVariantMap {
pub fn Find_0(&self) -> usize {
    // QVariantMap_find_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_find_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::const_iterator find(const Key &)
impl QVariantMap {
pub fn Find_1(&self) -> usize {
    // QVariantMap_find_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_find_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::const_iterator constFind(const Key &)
impl QVariantMap {
pub fn ConstFind_0(&self) -> usize {
    // QVariantMap_constFind_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_constFind_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::iterator lowerBound(const Key &)
impl QVariantMap {
pub fn LowerBound_0(&self) -> usize {
    // QVariantMap_lowerBound_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_lowerBound_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::const_iterator lowerBound(const Key &)
impl QVariantMap {
pub fn LowerBound_1(&self) -> usize {
    // QVariantMap_lowerBound_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_lowerBound_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::iterator upperBound(const Key &)
impl QVariantMap {
pub fn UpperBound_0(&self) -> usize {
    // QVariantMap_upperBound_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_upperBound_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::const_iterator upperBound(const Key &)
impl QVariantMap {
pub fn UpperBound_1(&self) -> usize {
    // QVariantMap_upperBound_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_upperBound_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::iterator insert(const Key &, const T &)
impl QVariantMap {
pub fn Insert_0(&self) -> usize {
    // QVariantMap_insert_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_insert_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::iterator insert(QMap::const_iterator, const Key &, const T &)
impl QVariantMap {
pub fn Insert_1(&self) -> usize {
    // QVariantMap_insert_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_insert_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::iterator insertMulti(const Key &, const T &)
impl QVariantMap {
pub fn InsertMulti_0(&self) -> usize {
    // QVariantMap_insertMulti_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_insertMulti_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap::iterator insertMulti(QMap::const_iterator, const Key &, const T &)
impl QVariantMap {
pub fn InsertMulti_1(&self) -> usize {
    // QVariantMap_insertMulti_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_insertMulti_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QMap<Key, T> & unite(const QMap<Key, T> &)
impl QVariantMap {
pub fn Unite_0(&self) -> usize {
    // QVariantMap_unite_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_unite_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// bool empty()
impl QVariantMap {
pub fn Empty_0(&self) -> bool {
    // QVariantMap_empty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_empty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QPair<QMap::iterator, QMap::iterator> equal_range(const Key &)
impl QVariantMap {
pub fn Equal_range_0(&self) -> usize {
    // QVariantMap_equal_range_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_equal_range_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QPair<QMap::const_iterator, QMap::const_iterator> equal_range(const Key &)
impl QVariantMap {
pub fn Equal_range_1(&self) -> usize {
    // QVariantMap_equal_range_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_equal_range_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void detach_helper()
impl QVariantMap {
pub fn Detach_helper_0(&self) -> (/*void*/) {
    // QVariantMap_detach_helper_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_detach_helper_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isValidIterator(const QMap::const_iterator &)
impl QVariantMap {
pub fn IsValidIterator_0(&self) -> bool {
    // QVariantMap_isValidIterator_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QVariantMap_isValidIterator_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
//  body block end
