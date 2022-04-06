; ModuleID = 'test.bc'
source_filename = "test"

%context = type { [8 x i32], [8 x i8] }

define fastcc void @indirect_bb_call(%context* %0, i8* %1, i32 %2) {
entry:
  switch i32 %2, label %not_found [
    i32 4198400, label %bb_00401000
    i32 4218880, label %bb_00406000
  ]

not_found:                                        ; preds = %entry
  call void @llvm.trap()
  ret void

bb_00401000:                                      ; preds = %entry
  tail call fastcc void @sub_00401000(%context* %0, i8* %1)
  ret void

bb_00406000:                                      ; preds = %entry
  tail call fastcc void @sub_00406000(%context* %0, i8* %1)
  ret void
}

define fastcc void @sub_00401000(%context* %0, i8* %1) {
entry:
  %EBP_ptr = getelementptr %context, %context* %0, i32 0, i32 0, i32 5
  %EBP = load i32, i32* %EBP_ptr, align 4
  %ESP_ptr = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  %ESP = load i32, i32* %ESP_ptr, align 4
  %2 = sub i32 %ESP, 4
  %ESP_ptr1 = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  store i32 %2, i32* %ESP_ptr1, align 4
  %3 = zext i32 %2 to i64
  %hptr = getelementptr i8, i8* %1, i64 %3
  %4 = bitcast i8* %hptr to i32*
  store i32 %EBP, i32* %4, align 1
  br label %bb_00401001

bb_00401001:                                      ; preds = %entry
  %ESP_ptr2 = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  %ESP3 = load i32, i32* %ESP_ptr2, align 4
  %EBP_ptr4 = getelementptr %context, %context* %0, i32 0, i32 0, i32 5
  store i32 %ESP3, i32* %EBP_ptr4, align 4
  br label %bb_00401003

bb_00401003:                                      ; preds = %bb_00401001
  %ESP_ptr5 = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  %ESP6 = load i32, i32* %ESP_ptr5, align 4
  %5 = sub i32 %ESP6, 24
  %ESP_ptr7 = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  store i32 %5, i32* %ESP_ptr7, align 4
  %6 = call { i32, i1 } @llvm.ssub.with.overflow.i32(i32 %ESP6, i32 24)
  %7 = extractvalue { i32, i1 } %6, 1
  %8 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 %ESP6, i32 24)
  %9 = extractvalue { i32, i1 } %8, 1
  %10 = icmp eq i32 %5, 0
  %flag_Zero_ptr = getelementptr %context, %context* %0, i32 0, i32 1, i32 3
  %11 = zext i1 %10 to i8
  store i8 %11, i8* %flag_Zero_ptr, align 1
  %12 = lshr i32 %5, 31
  %13 = trunc i32 %12 to i1
  %flag_Sign_ptr = getelementptr %context, %context* %0, i32 0, i32 1, i32 4
  %14 = zext i1 %13 to i8
  store i8 %14, i8* %flag_Sign_ptr, align 1
  %flag_Overflow_ptr = getelementptr %context, %context* %0, i32 0, i32 1, i32 5
  %15 = zext i1 %7 to i8
  store i8 %15, i8* %flag_Overflow_ptr, align 1
  %flag_Carry_ptr = getelementptr %context, %context* %0, i32 0, i32 1, i32 0
  %16 = zext i1 %9 to i8
  store i8 %16, i8* %flag_Carry_ptr, align 1
  br label %bb_00401006

bb_00401006:                                      ; preds = %bb_00401003
  %ESP_ptr8 = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  %ESP9 = load i32, i32* %ESP_ptr8, align 4
  %17 = add i32 12, %ESP9
  %18 = zext i32 %17 to i64
  %hptr10 = getelementptr i8, i8* %1, i64 %18
  %19 = bitcast i8* %hptr10 to i32*
  store i32 0, i32* %19, align 1
  br label %bb_0040100e

bb_0040100e:                                      ; preds = %bb_00401006
  %ESP_ptr11 = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  %ESP12 = load i32, i32* %ESP_ptr11, align 4
  %20 = add i32 8, %ESP12
  %21 = zext i32 %20 to i64
  %hptr13 = getelementptr i8, i8* %1, i64 %21
  %22 = bitcast i8* %hptr13 to i32*
  store i32 4202496, i32* %22, align 1
  br label %bb_00401016

bb_00401016:                                      ; preds = %bb_0040100e
  %ESP_ptr14 = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  %ESP15 = load i32, i32* %ESP_ptr14, align 4
  %23 = add i32 4, %ESP15
  %24 = zext i32 %23 to i64
  %hptr16 = getelementptr i8, i8* %1, i64 %24
  %25 = bitcast i8* %hptr16 to i32*
  store i32 4202514, i32* %25, align 1
  br label %bb_0040101e

bb_0040101e:                                      ; preds = %bb_00401016
  %ESP_ptr17 = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  %ESP18 = load i32, i32* %ESP_ptr17, align 4
  %26 = add i32 0, %ESP18
  %27 = zext i32 %26 to i64
  %hptr19 = getelementptr i8, i8* %1, i64 %27
  %28 = bitcast i8* %hptr19 to i32*
  store i32 0, i32* %28, align 1
  br label %bb_00401025

bb_00401025:                                      ; preds = %bb_0040101e
  %hptr20 = getelementptr i8, i8* %1, i64 4210736
  %29 = bitcast i8* %hptr20 to i32*
  %30 = load i32, i32* %29, align 1
  %EAX_ptr = getelementptr %context, %context* %0, i32 0, i32 0, i32 0
  store i32 %30, i32* %EAX_ptr, align 4
  br label %bb_0040102a

bb_0040102a:                                      ; preds = %bb_00401025
  %ESP_ptr21 = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  %ESP22 = load i32, i32* %ESP_ptr21, align 4
  %31 = sub i32 %ESP22, 4
  %ESP_ptr23 = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  store i32 %31, i32* %ESP_ptr23, align 4
  %32 = zext i32 %31 to i64
  %hptr24 = getelementptr i8, i8* %1, i64 %32
  %33 = bitcast i8* %hptr24 to i32*
  store i32 4198444, i32* %33, align 1
  %EAX_ptr25 = getelementptr %context, %context* %0, i32 0, i32 0, i32 0
  %EAX = load i32, i32* %EAX_ptr25, align 4
  call fastcc void @indirect_bb_call(%context* %0, i8* %1, i32 %EAX)
  br label %bb_0040102c

bb_0040102c:                                      ; preds = %bb_0040102a
  %ESP_ptr26 = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  %ESP27 = load i32, i32* %ESP_ptr26, align 4
  %34 = sub i32 %ESP27, 16
  %ESP_ptr28 = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  store i32 %34, i32* %ESP_ptr28, align 4
  %35 = call { i32, i1 } @llvm.ssub.with.overflow.i32(i32 %ESP27, i32 16)
  %36 = extractvalue { i32, i1 } %35, 1
  %37 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 %ESP27, i32 16)
  %38 = extractvalue { i32, i1 } %37, 1
  %39 = icmp eq i32 %34, 0
  %flag_Zero_ptr29 = getelementptr %context, %context* %0, i32 0, i32 1, i32 3
  %40 = zext i1 %39 to i8
  store i8 %40, i8* %flag_Zero_ptr29, align 1
  %41 = lshr i32 %34, 31
  %42 = trunc i32 %41 to i1
  %flag_Sign_ptr30 = getelementptr %context, %context* %0, i32 0, i32 1, i32 4
  %43 = zext i1 %42 to i8
  store i8 %43, i8* %flag_Sign_ptr30, align 1
  %flag_Overflow_ptr31 = getelementptr %context, %context* %0, i32 0, i32 1, i32 5
  %44 = zext i1 %36 to i8
  store i8 %44, i8* %flag_Overflow_ptr31, align 1
  %flag_Carry_ptr32 = getelementptr %context, %context* %0, i32 0, i32 1, i32 0
  %45 = zext i1 %38 to i8
  store i8 %45, i8* %flag_Carry_ptr32, align 1
  br label %bb_0040102f

bb_0040102f:                                      ; preds = %bb_0040102c
  %EAX_ptr33 = getelementptr %context, %context* %0, i32 0, i32 0, i32 0
  store i32 0, i32* %EAX_ptr33, align 4
  br label %bb_00401034

bb_00401034:                                      ; preds = %bb_0040102f
  %EBP_ptr34 = getelementptr %context, %context* %0, i32 0, i32 0, i32 5
  %EBP35 = load i32, i32* %EBP_ptr34, align 4
  %ESP_ptr36 = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  store i32 %EBP35, i32* %ESP_ptr36, align 4
  %ESP_ptr37 = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  %ESP38 = load i32, i32* %ESP_ptr37, align 4
  %46 = zext i32 %ESP38 to i64
  %hptr39 = getelementptr i8, i8* %1, i64 %46
  %47 = bitcast i8* %hptr39 to i32*
  %48 = load i32, i32* %47, align 1
  %49 = add i32 %ESP38, 4
  %ESP_ptr40 = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  store i32 %49, i32* %ESP_ptr40, align 4
  %EBP_ptr41 = getelementptr %context, %context* %0, i32 0, i32 0, i32 5
  store i32 %48, i32* %EBP_ptr41, align 4
  br label %bb_00401035

bb_00401035:                                      ; preds = %bb_00401034
  %ESP_ptr42 = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  %ESP43 = load i32, i32* %ESP_ptr42, align 4
  %50 = zext i32 %ESP43 to i64
  %hptr44 = getelementptr i8, i8* %1, i64 %50
  %51 = bitcast i8* %hptr44 to i32*
  %52 = load i32, i32* %51, align 1
  %53 = add i32 %ESP43, 4
  %ESP_ptr45 = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  store i32 %53, i32* %ESP_ptr45, align 4
  ret void
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.ssub.with.overflow.i32(i32, i32) #0

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.usub.with.overflow.i32(i32, i32) #0

define fastcc void @sub_00406000(%context* %0, i8* %1) {
entry:
  %ESP_ptr = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  %ESP = load i32, i32* %ESP_ptr, align 4
  %2 = sub i32 %ESP, 4
  %ESP_ptr1 = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  store i32 %2, i32* %ESP_ptr1, align 4
  %3 = zext i32 %2 to i64
  %hptr = getelementptr i8, i8* %1, i64 %3
  %4 = bitcast i8* %hptr to i32*
  store i32 4218887, i32* %4, align 1
  call fastcc void @magic_MessageBoxA(%context* %0, i8* %1)
  br label %bb_00406007

bb_00406007:                                      ; preds = %entry
  %ESP_ptr2 = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  %ESP3 = load i32, i32* %ESP_ptr2, align 4
  %5 = zext i32 %ESP3 to i64
  %hptr4 = getelementptr i8, i8* %1, i64 %5
  %6 = bitcast i8* %hptr4 to i32*
  %7 = load i32, i32* %6, align 1
  %8 = add i32 %ESP3, 4
  %ESP_ptr5 = getelementptr %context, %context* %0, i32 0, i32 0, i32 4
  store i32 %8, i32* %ESP_ptr5, align 4
  ret void
}

declare fastcc void @magic_MessageBoxA(%context*, i8*)

; Function Attrs: cold noreturn nounwind
declare void @llvm.trap() #1

attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #1 = { cold noreturn nounwind }
