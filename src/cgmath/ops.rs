// Copyright 2013 The CGMath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Scalar ops

pub trait AddS<S, Result> { fn add_s(&self, s: &S) -> Result; }
pub trait SubS<S, Result> { fn sub_s(&self, s: &S) -> Result; }
pub trait MulS<S, Result> { fn mul_s(&self, s: &S) -> Result; }
pub trait DivS<S, Result> { fn div_s(&self, s: &S) -> Result; }
pub trait RemS<S, Result> { fn rem_s(&self, s: &S) -> Result; }

#[inline] fn add_s<T: AddS<S, Result>, S, Result>(t: T, s: S) -> Result { t.add_s(&s) }
#[inline] fn sub_s<T: SubS<S, Result>, S, Result>(t: T, s: S) -> Result { t.sub_s(&s) }
#[inline] fn mul_s<T: MulS<S, Result>, S, Result>(t: T, s: S) -> Result { t.mul_s(&s) }
#[inline] fn div_s<T: DivS<S, Result>, S, Result>(t: T, s: S) -> Result { t.div_s(&s) }
#[inline] fn rem_s<T: RemS<S, Result>, S, Result>(t: T, s: S) -> Result { t.rem_s(&s) }

pub trait AddSelfS<S> { fn add_self_s(&mut self, s: &S); }
pub trait SubSelfS<S> { fn sub_self_s(&mut self, s: &S); }
pub trait MulSelfS<S> { fn mul_self_s(&mut self, s: &S); }
pub trait DivSelfS<S> { fn div_self_s(&mut self, s: &S); }
pub trait RemSelfS<S> { fn rem_self_s(&mut self, s: &S); }

// Angle ops

pub trait AddA<A, Result> { fn add_a(&self, a: &A) -> Result; }
pub trait SubA<A, Result> { fn sub_a(&self, a: &A) -> Result; }
pub trait MulA<A, Result> { fn mul_a(&self, a: &A) -> Result; }
pub trait DivA<A, Result> { fn div_a(&self, a: &A) -> Result; }
pub trait RemA<A, Result> { fn rem_a(&self, a: &A) -> Result; }

#[inline] fn add_a<T: AddA<A, Result>, A, Result>(t: T, a: A) -> Result { t.add_a(&a) }
#[inline] fn sub_a<T: SubA<A, Result>, A, Result>(t: T, a: A) -> Result { t.sub_a(&a) }
#[inline] fn mul_a<T: MulA<A, Result>, A, Result>(t: T, a: A) -> Result { t.mul_a(&a) }
#[inline] fn div_a<T: DivA<A, Result>, A, Result>(t: T, a: A) -> Result { t.div_a(&a) }
#[inline] fn rem_a<T: RemA<A, Result>, A, Result>(t: T, a: A) -> Result { t.rem_a(&a) }

pub trait AddSelfA<A> { fn add_self_a(&mut self, a: &A); }
pub trait SubSelfA<A> { fn sub_self_a(&mut self, a: &A); }
pub trait MulSelfA<A> { fn mul_self_a(&mut self, a: &A); }
pub trait DivSelfA<A> { fn div_self_a(&mut self, a: &A); }
pub trait RemSelfA<A> { fn rem_self_a(&mut self, a: &A); }

// Vector ops

pub trait AddV<V, Result> { fn add_v(&self, v: &V) -> Result; }
pub trait SubV<V, Result> { fn sub_v(&self, v: &V) -> Result; }
pub trait MulV<V, Result> { fn mul_v(&self, v: &V) -> Result; }
pub trait DivV<V, Result> { fn div_v(&self, v: &V) -> Result; }
pub trait RemV<V, Result> { fn rem_v(&self, v: &V) -> Result; }

#[inline] fn add_v<T: AddV<V, Result>, V, Result>(t: T, v: V) -> Result { t.add_v(&v) }
#[inline] fn sub_v<T: SubV<V, Result>, V, Result>(t: T, v: V) -> Result { t.sub_v(&v) }
#[inline] fn mul_v<T: MulV<V, Result>, V, Result>(t: T, v: V) -> Result { t.mul_v(&v) }
#[inline] fn div_v<T: DivV<V, Result>, V, Result>(t: T, v: V) -> Result { t.div_v(&v) }
#[inline] fn rem_v<T: RemV<V, Result>, V, Result>(t: T, v: V) -> Result { t.rem_v(&v) }

pub trait AddSelfV<V> { fn add_self_v(&mut self, v: &V); }
pub trait SubSelfV<V> { fn sub_self_v(&mut self, v: &V); }
pub trait MulSelfV<V> { fn mul_self_v(&mut self, v: &V); }
pub trait DivSelfV<V> { fn div_self_v(&mut self, v: &V); }
pub trait RemSelfV<V> { fn rem_self_v(&mut self, v: &V); }

// Point ops

pub trait AddP<P, Result> { fn add_p(&self, p: &P) -> Result; }
pub trait SubP<P, Result> { fn sub_p(&self, p: &P) -> Result; }
pub trait MulP<P, Result> { fn mul_p(&self, p: &P) -> Result; }
pub trait DivP<P, Result> { fn div_p(&self, p: &P) -> Result; }
pub trait RemP<P, Result> { fn rem_p(&self, p: &P) -> Result; }

#[inline] fn add_p<T: AddP<P, Result>, P, Result>(t: T, p: P) -> Result { t.add_p(&p) }
#[inline] fn sub_p<T: SubP<P, Result>, P, Result>(t: T, p: P) -> Result { t.sub_p(&p) }
#[inline] fn mul_p<T: MulP<P, Result>, P, Result>(t: T, p: P) -> Result { t.mul_p(&p) }
#[inline] fn div_p<T: DivP<P, Result>, P, Result>(t: T, p: P) -> Result { t.div_p(&p) }
#[inline] fn rem_p<T: RemP<P, Result>, P, Result>(t: T, p: P) -> Result { t.rem_p(&q) }

pub trait AddSelfP<P> { fn add_self_p(&mut self, p: &P); }
pub trait SubSelfP<P> { fn sub_self_p(&mut self, p: &P); }
pub trait MulSelfP<P> { fn mul_self_p(&mut self, p: &P); }
pub trait DivSelfP<P> { fn div_self_p(&mut self, p: &P); }
pub trait RemSelfP<P> { fn rem_self_p(&mut self, p: &P); }

// Matrix ops

pub trait AddM<M, Result> { fn add_m(&self, m: &M) -> Result; }
pub trait SubM<M, Result> { fn sub_m(&self, m: &M) -> Result; }
pub trait MulM<M, Result> { fn mul_m(&self, m: &M) -> Result; }
pub trait DivM<M, Result> { fn div_m(&self, m: &M) -> Result; }
pub trait RemM<M, Result> { fn rem_m(&self, m: &M) -> Result; }

#[inline] fn add_m<T: AddM<M, Result>, M, Result>(t: T, m: M) -> Result { t.add_m(&m) }
#[inline] fn sub_m<T: SubM<M, Result>, M, Result>(t: T, m: M) -> Result { t.sub_m(&m) }
#[inline] fn mul_m<T: MulM<M, Result>, M, Result>(t: T, m: M) -> Result { t.mul_m(&m) }
#[inline] fn div_m<T: DivM<M, Result>, M, Result>(t: T, m: M) -> Result { t.div_m(&m) }
#[inline] fn rem_m<T: RemM<M, Result>, M, Result>(t: T, m: M) -> Result { t.rem_m(&m) }

pub trait AddSelfM<M> { fn add_self_m(&mut self, m: &M); }
pub trait SubSelfM<M> { fn sub_self_m(&mut self, m: &M); }
pub trait MulSelfM<M> { fn mul_self_m(&mut self, m: &M); }
pub trait DivSelfM<M> { fn div_self_m(&mut self, m: &M); }
pub trait RemSelfM<M> { fn rem_self_m(&mut self, m: &M); }

// Quaternion ops

pub trait AddQ<Q, Result> { fn add_q(&self, q: &Q) -> Result; }
pub trait SubQ<Q, Result> { fn sub_q(&self, q: &Q) -> Result; }
pub trait MulQ<Q, Result> { fn mul_q(&self, q: &Q) -> Result; }
pub trait DivQ<Q, Result> { fn div_q(&self, q: &Q) -> Result; }
pub trait RemQ<Q, Result> { fn rem_q(&self, q: &Q) -> Result; }

#[inline] fn add_q<T: AddQ<Q, Result>, Q, Result>(t: T, q: Q) -> Result { t.add_q(&q) }
#[inline] fn sub_q<T: SubQ<Q, Result>, Q, Result>(t: T, q: Q) -> Result { t.sub_q(&q) }
#[inline] fn mul_q<T: MulQ<Q, Result>, Q, Result>(t: T, q: Q) -> Result { t.mul_q(&q) }
#[inline] fn div_q<T: DivQ<Q, Result>, Q, Result>(t: T, q: Q) -> Result { t.div_q(&q) }
#[inline] fn rem_q<T: RemQ<Q, Result>, Q, Result>(t: T, q: Q) -> Result { t.rem_q(&q) }

pub trait AddSelfQ<Q> { fn add_self_q(&mut self, q: &Q); }
pub trait SubSelfQ<Q> { fn sub_self_q(&mut self, q: &Q); }
pub trait MulSelfQ<Q> { fn mul_self_q(&mut self, q: &Q); }
pub trait DivSelfQ<Q> { fn div_self_q(&mut self, q: &Q); }
pub trait RemSelfQ<Q> { fn rem_self_q(&mut self, q: &Q); }
