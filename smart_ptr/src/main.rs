mod entity;
mod trait_obj;
mod smptr_box;
mod smptr_deref;
mod smptr_drop;
mod smptr_Rc_and_Arc;
mod smptr_cell_and_refcell;

use entity::Getter;


fn main() {
    let a: entity::Entity = entity::Entity::new("张三".to_string(), 32, "一个老师".to_string());
    let b = a;

    println!("{}", b.get_name());

    
    // println!("{}", a);  //Entity::name拥有堆区数据，拷贝时有所有权转移
    println!("{}", b);

    let c = entity::NoOwnerShip{real_part: 10, vir_part: -10};
    let d = c;

    // println!("{:?}", c);    //虽然NoOwnerShip没有堆区数据，但它没有实现clone特征，所以仍有所有权转移
    println!("{:?}", d);

    let ani_a = entity::Animal::new("猪".to_string(), 32);
    println!("{:#?}", ani_a);


}
