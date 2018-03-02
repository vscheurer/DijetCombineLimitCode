#!/bin/bash

# # # # 
# cd ../ExoDiBosonAnalysis
# # # # 
# # # # # # # # python MakeInput_btagseverywhere.py 1200
# python MakeInput_8cat_wob.py 1400
# python MakeInput_8cat_wob.py 1600
# python MakeInput_8cat_wob.py 1800
# python MakeInput_8cat_wob.py 2000
# python MakeInput_8cat_wob.py 2500
# python MakeInput_8cat_wob.py 3000
# python MakeInput_8cat_wob.py 3500
# python MakeInput_8cat_wob.py 4000
# python MakeInput_8cat_wob.py QCD
# # # # 
# cd ../DijetCombineLimitCode
# # # # # python interpolateVV13TeV.py BulkGtoZZ_bb_ 1200 GeV
# # # # # # # python interpolateVV13TeV.py BulkGtoZZ_bb_ 1300 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 1400 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 1500 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 1600 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 1700 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 1800 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 1900 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 2000 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 2100 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 2200 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 2300 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 2400 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 2500 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 2600 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 2700 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 2800 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 2900 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 3000 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 3100 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 3200 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 3300 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 3400 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 3500 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 3600 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 3700 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 3800 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 3900 GeV
# python interpolateVV13TeV_8cat.py BulkGtoZZ_bb_ 4000 GeV
# # # 
# root -b -q MiniTreeSignalProducerVV13TeV.C
# root -b -q MiniTreeProducerVV13TeV.C            ##CHANGE IF DATA!!
# # # # # # # # 
# # # # # # # 
# # # # # # # 
# # # # # # # 
# # # # # # # # 
# # # # # # # # # 
# # # # # echo "LOESCH MCIH!!"
# # # # # root -b -q "X2VVFitter.cc(1200,4,4,0)"
# # root -b -q "X2VVFitter.cc(1400,4,4,0)"
# # root -b -q "X2VVFitter.cc(1500,4,4,0)"
# # root -b -q "X2VVFitter.cc(1600,4,4,0)"
# # root -b -q "X2VVFitter.cc(1700,4,4,0)"
# # root -b -q "X2VVFitter.cc(1800,4,4,0)"
# # root -b -q "X2VVFitter.cc(1900,4,4,0)"
# # root -b -q "X2VVFitter.cc(2000,4,4,0)"
# # root -b -q "X2VVFitter.cc(2100,4,4,0)"
# # root -b -q "X2VVFitter.cc(2200,4,4,0)"
# # root -b -q "X2VVFitter.cc(2300,4,4,0)"
# # root -b -q "X2VVFitter.cc(2400,4,4,0)"
# # root -b -q "X2VVFitter.cc(2500,4,4,0)"
# # root -b -q "X2VVFitter.cc(2600,4,4,0)"
# # root -b -q "X2VVFitter.cc(2700,4,4,0)"
# # root -b -q "X2VVFitter.cc(2800,4,4,0)"
# # root -b -q "X2VVFitter.cc(2900,4,4,0)"
# # root -b -q "X2VVFitter.cc(3000,4,4,0)"
# # root -b -q "X2VVFitter.cc(3100,4,4,0)"
# # root -b -q "X2VVFitter.cc(3200,4,4,0)"
# # root -b -q "X2VVFitter.cc(3300,4,4,0)"
# # root -b -q "X2VVFitter.cc(3400,4,4,0)"
# # root -b -q "X2VVFitter.cc(3500,4,4,0)"
# # root -b -q "X2VVFitter.cc(3600,4,4,0)"
# # root -b -q "X2VVFitter.cc(3700,4,4,0)"
# # root -b -q "X2VVFitter.cc(3800,4,4,0)"
# # root -b -q "X2VVFitter.cc(3900,4,4,0)"
# # root -b -q "X2VVFitter.cc(4000,4,4,0)"
# cd ../job_submission/local/sframe/
# python batchsubmission.py --workspaces
# cd ../../../DijetCombineLimitCode
# 
# count2=0;
# 
# 
# while [ $count2 -ne 378 ];
# do
# count2=0;
# for name in `ls datacards/`
# do
#     
#     if [[ ! -e $name ]]
#     then
# 
#         count2=$(($count2+1))
#     fi
# done
# sleep 1
# echo $count2
# done
#  
#  
python implement-tau21PtUnc.py 
python implement-tau21SFUnc.py
python implement-JESJMRsystematics.py

python implement-btagUnc.py

cd Limits
python CombineDatacards.py

cd ..

combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_1400_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 1400 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_1500_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 1500 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_1600_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 1600 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_1700_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 1700 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_1800_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 1800 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_1900_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 1900 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_2000_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 2000 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_2100_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 2100 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_2200_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 2200 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_2300_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 2300 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_2400_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 2400 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_2500_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 2500 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_2600_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 2600 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_2700_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 2700 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_2800_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 2800 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_2900_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 2900 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_3000_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 3000 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_3100_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 3100 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_3200_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 3200 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_3300_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 3300 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_3400_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 3400 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_3500_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 3500 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_3600_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 3600 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_3700_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 3700 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_3800_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 3800 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_3900_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 3900 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01
combine /usr/users/vscheurer/DijetCombineLimitCode/datacards/CMS_jj_BulkZZ_4000_13TeV_CMS_jj_comb_bb.txt  -M Asymptotic -m 4000 -n nobtag --rMax 100 --rMin 0.000001 --minimizerTolerance 0.01

# # 
# # # 
# cd ../job_submission/local/sframe/
# # python batchsubmission.py --combine
# cd ../../../DijetCombineLimitCode
# count=0;
# 
# 
# while [ $count -ne 27 ];
# do
# count=0;
# for name in `ls test/`
# do
#     
#     if [[ ! -e $name ]]
#     then
# 
#         count=$(($count+1))
#     fi
# done
# sleep 1
# echo $count
# done
# #  cd test
# mv *nobtag*.root /Limits/withoutPDFandScale
# cd ../Limits/
# python brazilianFlag_theoryUncBand.py
