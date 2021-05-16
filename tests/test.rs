use dyn_partial_eq::*;

#[test]
fn test_dyn_partial_eq() {
  #[dyn_partial_eq]
  trait SomeTrait {}

  #[derive(DynPartialEq, PartialEq)]
  struct A(usize);
  impl SomeTrait for A {}

  #[derive(DynPartialEq, PartialEq)]
  struct B((usize, usize));
  impl SomeTrait for B {}

  let boxed_a_zero: Box<dyn SomeTrait> = Box::new(A(0));
  let boxed_a_one: Box<dyn SomeTrait> = Box::new(A(1));
  let boxed_b: Box<dyn SomeTrait> = Box::new(B((1, 2)));

  assert_eq!(&boxed_a_zero == &boxed_a_zero, true);
  assert_eq!(&boxed_a_zero == &boxed_a_one, false);
  assert_eq!(&boxed_a_zero == &boxed_b, false);

  assert_eq!(&boxed_a_one == &boxed_a_zero, false);
  assert_eq!(&boxed_a_one == &boxed_a_one, true);
  assert_eq!(&boxed_a_one == &boxed_b, false);

  assert_eq!(&boxed_b == &boxed_a_zero, false);
  assert_eq!(&boxed_b == &boxed_a_one, false);
  assert_eq!(&boxed_b == &boxed_b, true);
}

#[test]
fn test_dyn_nested_partial_eq() {
  #[dyn_partial_eq]
  trait ParentTrait {}

  #[derive(DynPartialEq, PartialEq)]
  struct Parent {
    child: Box<dyn ChildTrait>,
  }
  impl ParentTrait for Parent {}

  #[dyn_partial_eq]
  trait ChildTrait {}

  #[derive(DynPartialEq, PartialEq)]
  struct Child(usize);
  impl ChildTrait for Child {}


  let c1_a: Box<dyn ChildTrait> = Box::new(Child(1));
  let c1_b: Box<dyn ChildTrait> = Box::new(Child(1));
  let c2: Box<dyn ChildTrait> = Box::new(Child(2));

  assert_eq!(&c1_a == &c1_a, true);
  assert_eq!(&c1_a == &c1_b, true);
  assert_eq!(&c1_a == &c2, false);

  let p1_a: Box<dyn ParentTrait> = Box::new(Parent { child: c1_a });
  let p1_b: Box<dyn ParentTrait> = Box::new(Parent { child: c1_b });
  let p2: Box<dyn ParentTrait> = Box::new(Parent { child: c2 });

  assert_eq!(&p1_a == &p1_a, true);
  assert_eq!(&p1_a == &p1_b, true);
  assert_eq!(&p1_a == &p2, false);
}
