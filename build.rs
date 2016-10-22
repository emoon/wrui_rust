fn main() {
    println!("cargo:rustc-link-search=framework=/Users/danielcollin/code/other/Qt/5.7/clang_64/lib");
    println!("cargo:rustc-link-search=/Users/danielcollin/code/wrui/t2-output/macosx-clang-debug-default");
    println!("cargo:rustc-link-lib=static=wrui_qt");
    println!("cargo:rustc-link-lib=framework=QtCore");
    println!("cargo:rustc-link-lib=framework=QtWidgets");
    println!("cargo:rustc-link-lib=framework=QtGui");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
