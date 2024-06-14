use shared_lib::*;

#[test]
fn create_shared_lib() {
    let lib_path = LibPath::new_no_path("calculator".into());
    unsafe {
        SharedLib::new(lib_path).unwrap();
    }
}
#[test]
#[should_panic]
fn create_shared_lib_fail() {
    let lib_path = LibPath::new_no_path("non_existent".into());
    unsafe {
        SharedLib::new(lib_path).unwrap();
    }
}
#[test]
fn get_fn_from_shared_lib() {
    let lib_path = LibPath::new_no_path("calculator".into());
    unsafe {
        let lib = SharedLib::new(lib_path).unwrap();
        lib.get_fn::<fn(usize, usize) -> usize>("add").unwrap();
    }
}
#[test]
#[should_panic]
fn get_fn_from_shared_lib_fail() {
    let lib_path = LibPath::new_no_path("calculator".into());
    unsafe {
        let lib = SharedLib::new(lib_path).unwrap();
        lib.get_fn::<fn(usize, usize) -> usize>("non_existent")
            .unwrap();
    }
}
#[test]
fn call_fn_from_shared_lib() {
    let lib_path = LibPath::new_no_path("calculator".into());
    unsafe {
        let lib = SharedLib::new(lib_path).unwrap();
        let add_fn = lib.get_fn::<fn(usize, usize) -> usize>("add").unwrap();
        let result = add_fn.run(1, 2);
        assert_eq!(result, 3);
    }
}