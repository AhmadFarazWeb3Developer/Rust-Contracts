fn main() {
    let x = 5;
    println!("address : {}", &x);
    println!("address : {:p}", &x);

    let mut a = 10; // a owns the value 10 — it's like owning a locker with something inside

    // === Begin: Mutable borrow ===
    let b = &mut a; // We give the only key to b to modify the locker
    // Now a (the owner) says: "Okay, I won’t touch it until b is done."

    *b += 10; // b uses the key to change what's inside: now it's 20

    // println!("a {:p}", &a);          ERROR if uncommented
    // println!("a {}", &a);           ERROR if uncommented
    
    // These lines would try to look inside the locker while b still has the key
    // Rust says: "No! You can't access it until b is finished using it."

    println!("b sees address: {:p}", b); // ✅ OK — b still has the key
    println!("b sees value: {}", b); // ✅ OK — b still has access

    // === End: Mutable borrow goes out of scope here ===

    // Now b is done — the key is returned, a is free again

    println!("a sees address: {:p}", &a); // ✅ OK — safe again
    println!("a sees value: {}", &a); // ✅ OK — safe again
}
