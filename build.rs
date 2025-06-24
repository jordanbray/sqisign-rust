fn main() {
    // This is an opinionated cmake build
    let dst = cmake::Config::new("the-sqisign")
        .define("SQISIGN_BUILD_TYPE", "broadwell")
        .define("GMP_LIBRARY", "MINI")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CMAKE_BUILD_PARALLEL_LEVEL", "8")
        .no_build_target(true)
        .build();
    // I'm sorry for this.
    //
    // I _did_ try to coax cmake into producing more useful data to
    // avoid this, but the only method I found was to create a shared
    // library, which is arguably worse.
    //
    // This is broken up into two sections.
    // 1. All the folders that the default `the-sqisign` project puts
    //    it's shared objects into.
    // 2. All the individual libraries, all statically linked.

    // Section 1: Folders

    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-search=native={}/build/src", dst.display());
    println!(
        "cargo:rustc-link-search=native={}/build/src/common/generic",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/ec/ref/lvl1",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/ec/ref/lvl3",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/ec/ref/lvl5",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/gf/broadwell/lvl1",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/gf/broadwell/lvl3",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/gf/broadwell/lvl5",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/hd/ref/lvl1",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/hd/ref/lvl3",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/hd/ref/lvl5",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/id2iso/ref/lvl1",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/id2iso/ref/lvl3",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/id2iso/ref/lvl5",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/mp/ref/generic",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/precomp/ref/lvl1",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/precomp/ref/lvl3",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/precomp/ref/lvl5",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/quaternion/ref/generic",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/signature/ref/lvl1",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/signature/ref/lvl3",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/signature/ref/lvl5",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/verification/ref/lvl1",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/verification/ref/lvl3",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/verification/ref/lvl5",
        dst.display()
    );

    // Section 2: Files
    println!("cargo:rustc-link-lib=static:+whole-archive=GMP");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_common_sys");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_quaternion_generic");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_mp_generic");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_gf_lvl1");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_gf_lvl3");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_gf_lvl5");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_precomp_lvl1");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_precomp_lvl3");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_precomp_lvl5");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_ec_lvl1");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_ec_lvl3");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_ec_lvl5");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_hd_lvl1");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_hd_lvl3");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_hd_lvl5");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_verification_lvl1");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_verification_lvl3");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_verification_lvl5");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_id2iso_lvl1");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_id2iso_lvl3");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_id2iso_lvl5");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_signature_lvl1");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_signature_lvl3");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_signature_lvl5");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_lvl5");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_lvl1");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_lvl5_nistapi");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_lvl1_nistapi");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_lvl3");
    println!("cargo:rustc-link-lib=static:+whole-archive=sqisign_lvl3_nistapi");
}
