#!/bin/bash

python InjectSignal.py 1400 1
python InjectSignal.py 1600 1
python InjectSignal.py 1800 1
python InjectSignal.py 2000 1
python InjectSignal.py 2500 1
python InjectSignal.py 3000 1
python InjectSignal.py 3500 1
python InjectSignal.py 4000 1

cd ../ExoDiBosonAnalysis
python MakeInput_FakeSignal.py QCD 1400
cd ../DijetCombineLimitCode
root -b -q QCDMiniTreeProducerVV13TeV.C
root -b -q "X2VVFitter.cc(1400,4,4,6)"
mv workspaces/CMS_jj_bkg_VV_13TeV.root workspacesFakeSignal0.6/CMS_jj_bkg_VV_13TeV_1400.root
                                                 
cd ../ExoDiBosonAnalysis                         
python MakeInput_FakeSignal.py QCD 1600          
cd ../DijetCombineLimitCode                      
root -b -q QCDMiniTreeProducerVV13TeV.C             
root -b -q "X2VVFitter.cc(1600,4,4,6)"           
mv workspaces/CMS_jj_bkg_VV_13TeV.root workspacesFakeSignal0.6/CMS_jj_bkg_VV_13TeV_1600.root
                                                 
                                                 
cd ../ExoDiBosonAnalysis                         
python MakeInput_FakeSignal.py QCD 1800          
cd ../DijetCombineLimitCode                      
root -b -q QCDMiniTreeProducerVV13TeV.C             
root -b -q "X2VVFitter.cc(1800,4,4,6)"           
mv workspaces/CMS_jj_bkg_VV_13TeV.root workspacesFakeSignal0.6/CMS_jj_bkg_VV_13TeV_1800.root
                                                 
                                                 
cd ../ExoDiBosonAnalysis                         
python MakeInput_FakeSignal.py QCD 2000          
cd ../DijetCombineLimitCode                      
root -b -q QCDMiniTreeProducerVV13TeV.C             
root -b -q "X2VVFitter.cc(2000,4,4,6)"           
mv workspaces/CMS_jj_bkg_VV_13TeV.root workspacesFakeSignal0.6/CMS_jj_bkg_VV_13TeV_2000.root
                                                 
                                                 
cd ../ExoDiBosonAnalysis                         
python MakeInput_FakeSignal.py QCD 2500          
cd ../DijetCombineLimitCode                      
root -b -q QCDMiniTreeProducerVV13TeV.C             
root -b -q "X2VVFitter.cc(2500,4,4,6)"           
mv workspaces/CMS_jj_bkg_VV_13TeV.root workspacesFakeSignal0.6/CMS_jj_bkg_VV_13TeV_2500.root
                                                 
                                                 
cd ../ExoDiBosonAnalysis                         
python MakeInput_FakeSignal.py QCD 3000          
cd ../DijetCombineLimitCode                      
root -b -q QCDMiniTreeProducerVV13TeV.C             
root -b -q "X2VVFitter.cc(3000,4,4,6)"           
mv workspaces/CMS_jj_bkg_VV_13TeV.root workspacesFakeSignal0.6/CMS_jj_bkg_VV_13TeV_3000.root
                                                 
                                                 
cd ../ExoDiBosonAnalysis                         
python MakeInput_FakeSignal.py QCD 3500          
cd ../DijetCombineLimitCode                      
root -b -q QCDMiniTreeProducerVV13TeV.C             
root -b -q "X2VVFitter.cc(3500,4,4,6)"           
mv workspaces/CMS_jj_bkg_VV_13TeV.root workspacesFakeSignal0.6/CMS_jj_bkg_VV_13TeV_3500.root
                                                 
cd ../ExoDiBosonAnalysis                         
python MakeInput_FakeSignal.py QCD 4000          
cd ../DijetCombineLimitCode                      
root -b -q QCDMiniTreeProducerVV13TeV.C             
root -b -q "X2VVFitter.cc(4000,4,4,6)"           
mv workspaces/CMS_jj_bkg_VV_13TeV.root workspacesFakeSignal0.6/CMS_jj_bkg_VV_13TeV_4000.root
