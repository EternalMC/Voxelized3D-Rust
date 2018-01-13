use generic_array::*;
use std::fmt::{Display, Formatter, Result};
use typenum::*;
use std::ops::*;
use num::Zero;
use std::cmp::*;
use std::fmt;
use std::fmt::Debug;
use std::borrow::{Borrow, BorrowMut};
use std::convert::{AsMut, AsRef};
use std::hash::*;

//TODO use nalgebra package instead !

//newtype for GenericArray, zero runtime cost
//'Vector' is mathematical vector
pub struct Vector<T,N : ArrayLength<T>>(pub GenericArray<T,N>);


pub struct Matrix<T,N,M> where
    N : Mul<M>,
    typenum::Prod<N,M> : ArrayLength<T>,{

        pub ar : GenericArray<T, typenum::Prod<N,M>>,
}

impl <T : alga, N, M> Matrix<T,N,M> where
    N : Mul<M>,
    typenum::Prod<N,M> : ArrayLength<T>,{
    

    fn new_empty() -> Matrix<T,N,M>{
        let ar = GenericArray::generate(|_| T::zero());
        Matrix{ar}
    }
}

//Not possible :( , so the above newtype ^^ is required
/*impl<T, N : ArrayLength<T>> GenericArray<T,N>{
    pub fn test(&self){
        println!("this is test !");
    }
}*/

impl <T, N : ArrayLength<T>> From<GenericArray<T,N>> for Vector<T,N>{
    fn from(ar: GenericArray<T,N>) -> Self {
        Vector(ar)
    }
}

impl<T, N : ArrayLength<T>> Vector<T,N>{

    pub fn get(&self) -> &GenericArray<T,N>{
        &self.0
    }
}


impl<T : Clone,N : ArrayLength<T>> Clone for Vector<T,N>{

    fn clone(&self) -> Vector<T,N>{
        Vector::from((&self.0).clone())
    }
}

impl<T: PartialEq, N> PartialEq for Vector<T, N>
    where
        N: ArrayLength<T>,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T: PartialOrd, N> PartialOrd for Vector<T, N>
    where
        N: ArrayLength<T>,
{
    #[inline]
    fn partial_cmp(&self, other: &Vector<T, N>) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T : Display, N : ArrayLength<T>> Display for Vector<T,N>{

    fn fmt(&self, f : &mut Formatter) -> Result{
        let _ = write!(f, "[");
        for i in 0..N::to_usize(){
            if i != N::to_usize() - 1{
                let _ = write!(f, "{}, ", self.get()[i]);
            }
            else{
                let _ =write!(f, "{}", self.get()[i]);
            }
        }
        write!(f, "]")
    }
}

impl<T: Debug, N> Debug for Vector<T, N>
    where
        N: ArrayLength<T>,
{

    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0[..].fmt(fmt)
    }
}

impl<T, N> Borrow<[T]> for Vector<T, N>
    where
        N: ArrayLength<T>,
{
    #[inline]
    fn borrow(&self) -> &[T] {
        &self.0[..]
    }
}


impl<T, N> BorrowMut<[T]> for Vector<T, N>
    where
        N: ArrayLength<T>,
{
    #[inline]
    fn borrow_mut(&mut self) -> &mut [T] {
        &mut self.0[..]
    }
}

impl<T, N> AsRef<[T]> for Vector<T, N>
    where
        N: ArrayLength<T>,
{
    #[inline]
    fn as_ref(&self) -> &[T] {
        &self.0[..]
    }
}

impl<T, N> AsMut<[T]> for Vector<T, N>
    where
        N: ArrayLength<T>,
{
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.0[..]
    }
}

impl<T: Hash, N> Hash for Vector<T, N>
    where
        N: ArrayLength<T>,
{
    #[inline]
    fn hash<H>(&self, state: &mut H)
        where
            H: Hasher,
    {
        self.0.hash(state)
    }
}

impl<T : Display,N> Vector<T, N> where N : ArrayLength<T>{
    pub fn print(&self){
      print!("[");
      for i in 0..N::to_usize(){
          if i != N::to_usize() - 1{
              print!("{}, ", self.get()[i]);
          }
          else{
              print!("{}", self.get()[i]);
          }
      }
      println!("]");
   }
}

impl<'a,
     'b,
     T : Add<Output=T> + Copy,
     N : ArrayLength<T>>
    
    Add<& 'b Vector<T,N>>
    
for & 'a Vector<T,N>{
    
    type Output = Vector<T,N>;

    fn add(self, other : & 'b Vector<T,N>) -> Vector<T,N>{
        Vector(GenericArray::<T,N>::
              generate(&|i| self.get()[i] + other.get()[i]))
    }

}

impl<'a,
     'b,
     T : Sub<Output=T> + Copy,
     N : ArrayLength<T>>
    
    Sub<& 'b Vector<T,N>>

for & 'a Vector<T,N>{
    
    type Output = Vector<T,N>;

    fn sub(self, other : & 'b Vector<T,N>) -> Vector<T,N>{
        Vector(GenericArray::<T,N>::
              generate(&|i| self.get()[i] - other.get()[i]))
    } 
}



impl<'a,
     'b,
     T : Mul<Output=T> + Add<Output=T> + Copy + Zero,
     N : ArrayLength<T>>
    
    Mul<& 'b Vector<T,N>>
    
for & 'a Vector<T,N>{
    
    type Output = T;

    fn mul(self, other : & 'b Vector<T,N>) -> T{

        let mut sum : T = <T as Zero>::zero();
        for i in 0.. <N>::to_i32(){
            sum = sum + self.get()[i as usize] * other.get()[i as usize];
        };

        sum
    } 
}




