; ModuleID = 'probe5.138d13db6ce1c5cf-cgu.0'
source_filename = "probe5.138d13db6ce1c5cf-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

@alloc_53f213024f84c9d7ee1934f674248115 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/20999de3a2f866a6006169c9bc188017aba79fcc/library/core/src/num/mod.rs" }>, align 1
@alloc_24dc75cd1a82453a1066c680d90d2bab = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_53f213024f84c9d7ee1934f674248115, [12 x i8] c"K\00\00\00w\04\00\00\05\00\00\00" }>, align 4
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe5::probe
; Function Attrs: nounwind
define hidden void @_ZN6probe55probe17hab18a4810e46a61cE() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17he552e5fcc1324db9E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hb3b1c7f19c959d91E(ptr align 1 @str.0, i32 25, ptr align 4 @alloc_24dc75cd1a82453a1066c680d90d2bab) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17he552e5fcc1324db9E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare hidden i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn nounwind
declare dso_local void @_ZN4core9panicking5panic17hb3b1c7f19c959d91E(ptr align 1, i32, ptr align 4) unnamed_addr #2

attributes #0 = { nounwind "target-cpu"="mvp" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn nounwind "target-cpu"="mvp" }
attributes #3 = { noreturn nounwind }

!llvm.ident = !{!0}

!0 = !{!"rustc version 1.74.0-nightly (20999de3a 2023-09-15)"}
