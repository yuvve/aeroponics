.ALIASES
_    Oscillator(Oscillator_Out=N27383 ) CN @PROJEKTKURS.SCHEMATIC1(sch_1):Oscillator@PROJEKTKURS.Oscillator(sch_1)
V_Oscillator_V11          Oscillator.V11(+=0 -=Oscillator_N25994 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):Oscillator@PROJEKTKURS.Oscillator(sch_1):INS25968@SOURCE.VDC.Normal(chips)
R_Oscillator_R16          Oscillator.R16(1=N27383 2=Oscillator_N25914 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):Oscillator@PROJEKTKURS.Oscillator(sch_1):INS26072@ANALOG.R.Normal(chips)
X_Oscillator_U2C          Oscillator.U2C(+=Oscillator_N26132 -=Oscillator_N25954 V+=Oscillator_N26140 V-=Oscillator_N25994
+OUT=N27383 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):Oscillator@PROJEKTKURS.Oscillator(sch_1):INS26038@TEX_INST.TL074/301/TI.Normal(chips)
R_Oscillator_R18          Oscillator.R18(1=Oscillator_N26222 2=N27383 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):Oscillator@PROJEKTKURS.Oscillator(sch_1):INS26192@ANALOG.R.Normal(chips)
R_Oscillator_POT3          Oscillator.POT3(1=Oscillator_N25914 2=Oscillator_N25954 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):Oscillator@PROJEKTKURS.Oscillator(sch_1):INS25920@ANALOG.R.Normal(chips)
C_Oscillator_C3          Oscillator.C3(1=0 2=Oscillator_N26132 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):Oscillator@PROJEKTKURS.Oscillator(sch_1):INS26244@ANALOG.C.Normal(chips)
X_Oscillator_D3          Oscillator.D3(AN=N27383 CAT=Oscillator_N259460 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):Oscillator@PROJEKTKURS.Oscillator(sch_1):INS26006@DIZ.1N4733.Normal(chips)
V_Oscillator_V12          Oscillator.V12(+=Oscillator_N26140 -=0 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):Oscillator@PROJEKTKURS.Oscillator(sch_1):INS26142@SOURCE.VDC.Normal(chips)
R_Oscillator_R19          Oscillator.R19(1=0 2=Oscillator_N26132 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):Oscillator@PROJEKTKURS.Oscillator(sch_1):INS26228@ANALOG.R.Normal(chips)
R_Oscillator_R17          Oscillator.R17(1=0 2=Oscillator_N25954 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):Oscillator@PROJEKTKURS.Oscillator(sch_1):INS26022@ANALOG.R.Normal(chips)
X_Oscillator_D2          Oscillator.D2(AN=Oscillator_N25914 CAT=Oscillator_N259460 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):Oscillator@PROJEKTKURS.Oscillator(sch_1):INS25936@DIZ.1N4733.Normal(chips)
C_Oscillator_C2          Oscillator.C2(1=Oscillator_N26132 2=Oscillator_N26222 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):Oscillator@PROJEKTKURS.Oscillator(sch_1):INS26176@ANALOG.C.Normal(chips)
_    _(Oscillator.Oscillator_Out=N27383)
_    Gain_Loop(Input=N27227 Output=N26873 ) CN @PROJEKTKURS.SCHEMATIC1(sch_1):Gain Loop@PROJEKTKURS.Gain Loop(sch_1)
R_Gain_Loop_R4          Gain_Loop.R4(1=Gain_Loop_N15008 2=N26873 ) CN @PROJEKTKURS.SCHEMATIC1(sch_1):Gain Loop@PROJEKTKURS.Gain
+Loop(sch_1):INS14638@ANALOG.R.Normal(chips)
R_Gain_Loop_R5          Gain_Loop.R5(1=0 2=Gain_Loop_N15008 ) CN @PROJEKTKURS.SCHEMATIC1(sch_1):Gain Loop@PROJEKTKURS.Gain
+Loop(sch_1):INS14654@ANALOG.R.Normal(chips)
R_Gain_Loop_R2          Gain_Loop.R2(1=N27227 2=Gain_Loop_N14878 ) CN @PROJEKTKURS.SCHEMATIC1(sch_1):Gain Loop@PROJEKTKURS.Gain
+Loop(sch_1):INS35@ANALOG.R.Normal(chips)
V_Gain_Loop_V2          Gain_Loop.V2(+=Gain_Loop_N14759 -=0 ) CN @PROJEKTKURS.SCHEMATIC1(sch_1):Gain Loop@PROJEKTKURS.Gain
+Loop(sch_1):INS14695@SOURCE.VDC.Normal(chips)
R_Gain_Loop_R3          Gain_Loop.R3(1=0 2=Gain_Loop_N14878 ) CN @PROJEKTKURS.SCHEMATIC1(sch_1):Gain Loop@PROJEKTKURS.Gain
+Loop(sch_1):INS61@ANALOG.R.Normal(chips)
V_Gain_Loop_V3          Gain_Loop.V3(+=0 -=Gain_Loop_N14983 ) CN @PROJEKTKURS.SCHEMATIC1(sch_1):Gain Loop@PROJEKTKURS.Gain
+Loop(sch_1):INS14711@SOURCE.VDC.Normal(chips)
X_Gain_Loop_U2          Gain_Loop.U2(+=Gain_Loop_N14878 -=Gain_Loop_N15008 V+=Gain_Loop_N14759 V-=Gain_Loop_N14983 OUT=N26873 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):Gain Loop@PROJEKTKURS.Gain Loop(sch_1):INS25275@EVAL.uA741.Normal(chips)
_    _(Gain_Loop.Input=N27227)
_    _(Gain_Loop.Output=N26873)
_    AC_to_DC_conv(input=N26873 output=N26834 ) CN @PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1)
V_AC_to_DC_conv_V5          AC_to_DC_conv.V5(+=AC_to_DC_conv_N21929 -=0 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1):INS21935@SOURCE.VDC.Normal(chips)
X_AC_to_DC_conv_U1B          AC_to_DC_conv.U1B(+=N26873 -=AC_to_DC_conv_N21661 V+=AC_to_DC_conv_N21929 V-=AC_to_DC_conv_N21791
+OUT=AC_to_DC_conv_N21671 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1):INS21805@TEX_INST.TL074/301/TI.Normal(chips)
X_AC_to_DC_conv_D1          AC_to_DC_conv.D1(PDC=AC_to_DC_conv_N21697 AC2=AC_to_DC_conv_N21671 NDC=AC_to_DC_conv_N21541
+AC1=AC_to_DC_conv_N21661 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1):INS21569@DIF.MDA2500.Normal(chips)
X_AC_to_DC_conv_U1C          AC_to_DC_conv.U1C(+=AC_to_DC_conv_N21915 -=AC_to_DC_conv_N21563 V+=AC_to_DC_conv_N21933
+V-=AC_to_DC_conv_N21775 OUT=N26834 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1):INS21839@TEX_INST.TL074/301/TI.Normal(chips)
V_AC_to_DC_conv_V10          AC_to_DC_conv.V10(+=0 -=AC_to_DC_conv_N22179 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1):INS22193@SOURCE.VDC.Normal(chips)
V_AC_to_DC_conv_V6          AC_to_DC_conv.V6(+=0 -=AC_to_DC_conv_N21791 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1):INS21745@SOURCE.VDC.Normal(chips)
R_AC_to_DC_conv_R8          AC_to_DC_conv.R8(1=AC_to_DC_conv_N21563 2=N26834 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1):INS21599@ANALOG.R.Normal(chips)
R_AC_to_DC_conv_R7          AC_to_DC_conv.R7(1=AC_to_DC_conv_N21697 2=AC_to_DC_conv_N21915 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1):INS21889@ANALOG.R.Normal(chips)
R_AC_to_DC_conv_R15          AC_to_DC_conv.R15(1=AC_to_DC_conv_N22045 2=AC_to_DC_conv_N22075 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1):INS22049@ANALOG.R.Normal(chips)
R_AC_to_DC_conv_R13          AC_to_DC_conv.R13(1=AC_to_DC_conv_N21915 2=AC_to_DC_conv_N22101 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1):INS22103@ANALOG.R.Normal(chips)
C_AC_to_DC_conv_C1          AC_to_DC_conv.C1(1=AC_to_DC_conv_N21697 2=AC_to_DC_conv_N21541 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1):INS21679@ANALOG.C.Normal(chips)
R_AC_to_DC_conv_POT1          AC_to_DC_conv.POT1(1=AC_to_DC_conv_N21491 2=0 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1):INS21463@ANALOG.R.Normal(chips)
V_AC_to_DC_conv_V7          AC_to_DC_conv.V7(+=AC_to_DC_conv_N21933 -=0 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1):INS21963@SOURCE.VDC.Normal(chips)
R_AC_to_DC_conv_R14          AC_to_DC_conv.R14(1=AC_to_DC_conv_N22179 2=AC_to_DC_conv_N22189 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1):INS22157@ANALOG.R.Normal(chips)
R_AC_to_DC_conv_POT2_1          AC_to_DC_conv.POT2_1(1=AC_to_DC_conv_N22101 2=AC_to_DC_conv_N22075 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1):INS22079@ANALOG.R.Normal(chips)
R_AC_to_DC_conv_POT2_2          AC_to_DC_conv.POT2_2(1=AC_to_DC_conv_N22189 2=AC_to_DC_conv_N22101 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1):INS22137@ANALOG.R.Normal(chips)
V_AC_to_DC_conv_V9          AC_to_DC_conv.V9(+=AC_to_DC_conv_N22045 -=0 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1):INS22023@SOURCE.VDC.Normal(chips)
R_AC_to_DC_conv_R10          AC_to_DC_conv.R10(1=AC_to_DC_conv_N21697 2=AC_to_DC_conv_N21541 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1):INS21619@ANALOG.R.Normal(chips)
R_AC_to_DC_conv_R6          AC_to_DC_conv.R6(1=AC_to_DC_conv_N21661 2=AC_to_DC_conv_N21491 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1):INS21519@ANALOG.R.Normal(chips)
R_AC_to_DC_conv_R9          AC_to_DC_conv.R9(1=AC_to_DC_conv_N21541 2=AC_to_DC_conv_N21563 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1):INS21499@ANALOG.R.Normal(chips)
V_AC_to_DC_conv_V8          AC_to_DC_conv.V8(+=0 -=AC_to_DC_conv_N21775 ) CN
+@PROJEKTKURS.SCHEMATIC1(sch_1):AC_to_DC_conv@PROJEKTKURS.AC_to_DC_conv(sch_1):INS21717@SOURCE.VDC.Normal(chips)
_    _(AC_to_DC_conv.input=N26873)
_    _(AC_to_DC_conv.output=N26834)
R_R1            R1(1=0 2=N26834 ) CN @PROJEKTKURS.SCHEMATIC1(sch_1):INS25933@ANALOG.R.Normal(chips)
V_V1            V1(+=N27227 -=0 ) CN @PROJEKTKURS.SCHEMATIC1(sch_1):INS27187@SOURCE.VPULSE.Normal(chips)
R_R2            R2(1=0 2=N27383 ) CN @PROJEKTKURS.SCHEMATIC1(sch_1):INS27588@ANALOG.R.Normal(chips)
.ENDALIASES
