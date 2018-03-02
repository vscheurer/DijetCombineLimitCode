#!/bin/bash

cd Limits
python CombineDatacards.py
cd ..

cp workspacesFakeSignal0.6/CMS_jj_bkg_VV_13TeV_1400.root workspaces/CMS_jj_bkg_VV_13TeV.root
combine datacards/CMS_jj_BulkZZ_1400_13TeV_CMS_jj_comb_bb.txt  -M ProfileLikelihood -m 1400 --signif -n FakeSignal



cp workspacesFakeSignal0.6/CMS_jj_bkg_VV_13TeV_1600.root workspaces/CMS_jj_bkg_VV_13TeV.root
 combine datacards/CMS_jj_BulkZZ_1600_13TeV_CMS_jj_comb_bb.txt  -M ProfileLikelihood -m 1600 --signif -n FakeSignal
                                                                                                                  
                                                                                                                  
                                                                                                                  
                                                                                                                  
cp workspacesFakeSignal0.6/CMS_jj_bkg_VV_13TeV_1800.root workspaces/CMS_jj_bkg_VV_13TeV.root                      
 combine datacards/CMS_jj_BulkZZ_1800_13TeV_CMS_jj_comb_bb.txt  -M ProfileLikelihood -m 1800 --signif -n FakeSignal
                                                                                                                   
cp workspacesFakeSignal0.6/CMS_jj_bkg_VV_13TeV_2000.root workspaces/CMS_jj_bkg_VV_13TeV.root                       
 combine datacards/CMS_jj_BulkZZ_2000_13TeV_CMS_jj_comb_bb.txt  -M ProfileLikelihood -m 2000 --signif -n FakeSignal
                                                                                                                   
cp workspacesFakeSignal0.6/CMS_jj_bkg_VV_13TeV_2500.root workspaces/CMS_jj_bkg_VV_13TeV.root                       
 combine datacards/CMS_jj_BulkZZ_2500_13TeV_CMS_jj_comb_bb.txt  -M ProfileLikelihood -m 2500 --signif -n FakeSignal
                                                                                                                   
cp workspacesFakeSignal0.6/CMS_jj_bkg_VV_13TeV_3000.root workspaces/CMS_jj_bkg_VV_13TeV.root                       
 combine datacards/CMS_jj_BulkZZ_3000_13TeV_CMS_jj_comb_bb.txt  -M ProfileLikelihood -m 3000 --signif -n FakeSignal
                                                                                                                   
cp workspacesFakeSignal0.6/CMS_jj_bkg_VV_13TeV_3500.root workspaces/CMS_jj_bkg_VV_13TeV.root                       
 combine datacards/CMS_jj_BulkZZ_3500_13TeV_CMS_jj_comb_bb.txt  -M ProfileLikelihood -m 3500 --signif -n FakeSignal
                                                                                                                   
cp workspacesFakeSignal0.6/CMS_jj_bkg_VV_13TeV_4000.root workspaces/CMS_jj_bkg_VV_13TeV.root                       
 combine datacards/CMS_jj_BulkZZ_4000_13TeV_CMS_jj_comb_bb.txt  -M ProfileLikelihood -m 4000 --signif -n FakeSignal