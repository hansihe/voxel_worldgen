use num::Num;
use std::ops::{ Add, Sub };

#[derive(Copy, Clone, Hash, Default, Debug)]
pub struct Point2<T> {
    pub x: T,
    pub z: T,
}
#[derive(Copy, Clone, Hash, Default, Debug)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl <T: Num> Point2<T> {
    pub fn new(x: T, z: T) -> Point2<T> {
        Point2 { x: x, z: z }
    }
}
impl <T: Num> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Point3<T> {
        Point3 { x: x, y: y, z: z }
    }
}

macro_rules! impl_operator {
    (<$T:ident: $Constraint:ident> $Op:ident<$Rhs:ty> for $Lhs:ty {
        fn $op:ident($lhs: ident, $rhs: ident) -> $Output:ty { $body: expr }
    }) => {
        impl<$T: $Constraint> $Op<$Rhs> for $Lhs {
            type Output = $Output;
            fn $op(self, other: $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other); $body
            }
        }
        impl<'a, $T: $Constraint> $Op<&'a $Rhs> for $Lhs {
            type Output = $Output;
            fn $op(self, other: &'a $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other); $body
            }
        }
        impl<'b, $T: $Constraint> $Op<$Rhs> for &'b $Lhs {
            type Output = $Output;
            fn $op(self, other: $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other); $body
            }
        }
        impl<'a, 'b, $T: $Constraint> $Op<&'a $Rhs> for &'b $Lhs {
            type Output = $Output;
            fn $op(self, other: &'a $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other); $body
            }
        }
    }
}

macro_rules! impl_point {
    ($PointN:ident { $($field:ident),+ }, $n:expr) => {
        impl_operator!(<T: Num> Add<$PointN<T> > for $PointN<T> {
            fn add(lhs, rhs) -> $PointN<T> { $PointN::new($(lhs.$field + rhs.$field),+) }
        });
        //impl<T: Num+Copy> Add<$PointN<T>> for $PointN<T> {
        //    type Output = $PointN<T>;
        //    fn add(self, rhs: $PointN<T>) -> $PointN<T> {
        //        $PointN::new($(self.$field + rhs.$field),+)
        //    }
        //}
        //impl<T: Num+Copy> Add<T> for $PointN<T> {
        //    type Output = $PointN<T>;
        //    fn add(self, rhs: T) -> $PointN<T> {
        //        $PointN::new($(self.$field + rhs),+)
        //    }
        //}
    }
}

impl_point!(Point2 { x, z }, 2);
impl_point!(Point3 { x, y, z }, 3);
