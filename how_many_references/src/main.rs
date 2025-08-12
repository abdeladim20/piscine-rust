use how_many_references::*;

fn main() {
    let a = Rc::new("a".to_owned());
    let b = Rc::new("b".to_owned());
    let c = Rc::new("c".to_owned());

    let a1 = Rc::new("a".to_owned());

    let mut new_node = Node::new(vec![a.clone()]);
    new_node.add_element(b.clone());
    new_node.add_element(a.clone());
    new_node.add_element(c.clone());
    new_node.add_element(a.clone());
    new_node.add_element(a1.clone());
    new_node.add_element(a1.clone());
    println!("list: --- {:?}", new_node.ref_list);

    // println!("a: {:?}", how_many_references(&a));
    // println!("b: {:?}", how_many_references(&b));
    // println!("c: {:?}", how_many_references(&c));
    new_node.rm_all_ref(a1.clone());
    // new_node.rm_all_ref(a.clone());

    println!("a: {:?}", how_many_references(&a));
    println!("a1: {:?}", how_many_references(&a1));
    // println!("b: {:?}", how_many_references(&b));
    // println!("c: {:?}", how_many_references(&c));
}

/*
a: 4
b: 2
c: 2
a: 1
b: 2
c: 2
*/