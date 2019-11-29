use super::{PineStaticType, PineType, RuntimetErr};

pub fn downcast<'a, T: PineStaticType + 'a>(
    item: Box<dyn PineType<'a> + 'a>,
) -> Result<Box<T>, RuntimetErr> {
    if T::static_type() == item.get_type() {
        unsafe {
            let raw: *mut dyn PineType<'a> = Box::into_raw(item);
            Ok(Box::from_raw(raw as *mut T))
        }
    } else {
        Err(RuntimetErr::NotCompatible)
    }
}

pub fn downcast_ref<'a, T: PineStaticType + 'a>(
    item: &'a mut dyn PineType<'a>,
) -> Result<&mut T, RuntimetErr> {
    if T::static_type() == item.get_type() {
        unsafe {
            let raw: *mut dyn PineType<'a> = item;
            let t = raw as *mut T;
            Ok(t.as_mut().unwrap())
        }
    } else {
        Err(RuntimetErr::NotCompatible)
    }
}
