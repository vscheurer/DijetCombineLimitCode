from ROOT import *
import ROOT
import array, math
import os
from optparse import OptionParser
import sys



postfix = ""
#channels=["RS1WW","RS1ZZ","WZ","qW","qZ","BulkWW","BulkZZ"]
#channels=["BulkWW","BulkZZ","ZprimeWW","WZ"]#,"qW","qZ"]
channels=["BulkZZ"]
# channels=["qW","qZ"]


argv = sys.argv
parser = OptionParser()   
parser.add_option("-b", "--batch", dest="batch", default=False,action="store_true",
                              help="set batch mode")
parser.add_option("-s", "--signal", dest="signal", default="BulkZZ",action="store",
                              help="set signal. only in batch mode")
#parser.add_option("-m", "--mass", dest="mass", default=1200,action="store",
                              #help="set mass. only in batch mode")
parser.add_option("-p", "--path", dest="path",action="store", default="/usr/users/vscheurer/DijetCombineLimitCode/",
                              help="set input path")
(opts, args) = parser.parse_args(argv) 

postfix = ""
#channels=["WZ","BulkWW","BulkZZ","ZprimeWW","qW","qZ"]
channels=["BulkZZ"]
#channels=["WZ","ZprimeWW"]
#channels=["qW","qZ"]
outdir = opts.path+"datacards/"
if opts.batch:
    channels =[opts.signal]



for chan in channels:

    if "q" in chan:
       masses =[m*100/2 for m in range(2*10,2*40+1)]
       masses =[m*100 for m in range(12,62+1)]
    else:
       #masses =[m*50 for m in range(20,80+1)]
       masses =[m*100 for m in range(14,40+1)]

    for mass in masses:
        print "mass = ",mass

        ## bin0="CMS_jj_VVHP=/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_VVHP.txt "
        ## bin1="CMS_jj_VVLP=/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_VVLP.txt "
        ## bin3="CMS_jj_qVHP=/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_qVHP.txt "
        ## bin4="CMS_jj_qVLP=/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_qVLP.txt "
        
        #bin5="CMS_jj_WWHP=/usr/users/vscheurer/DijetCombineLimitCode/datacards/combinedCards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_WWHP.txt "
        #bin6="CMS_jj_WZHP=/usr/users/vscheurer/DijetCombineLimitCode/datacards/combinedCards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_WZHP.txt "
        #bin7="CMS_jj_ZZHP=/usr/users/vscheurer/DijetCombineLimitCode/datacards/combinedCards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_ZZHP.txt "
                                                                                                          
        #bin8="CMS_jj_WWLP=/usr/users/vscheurer/DijetCombineLimitCode/datacards/combinedCards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_WWLP.txt "
        #bin9="CMS_jj_WZLP=/usr/users/vscheurer/DijetCombineLimitCode/datacards/combinedCards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_WZLP.txt "
        #bin10="CMS_jj_ZZLP=/usr/users/vscheurer/DijetCombineLimitCode/datacards/combinedCards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_ZZLP.txt "
        
        #bin11="CMS_jj_qWHP=/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_qWHP.txt "
        #bin12="CMS_jj_qZHP=/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_qZHP.txt "
        
        #bin13="CMS_jj_qWLP=/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_qWLP.txt "
        #bin14="CMS_jj_qZLP=/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_qZLP.txt "
        
        
        ## bin01="/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_VV.txt "
        ## bin34="/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_qV.txt "
        
        #bin567="/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_VVHPnew.txt "
        #bin8910="/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_VVLPnew.txt "
        #bin5678910="/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_VVnew.txt "
        
        #bin1112="/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_qVHPnew.txt "
        #bin1314="/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_qVLPnew.txt "
        #bin11121314="/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_qVnew.txt "
        
        
        
        
        binZZHP="CMS_jj_ZZHP=/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_ZZHP.txt "
        binZZLP="CMS_jj_ZZLP=/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_ZZLP.txt "
        binWZHP="CMS_jj_WZHP=/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_WZHP.txt "
        binWZLP="CMS_jj_WZLP=/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_WZLP.txt "
        binWWHP="CMS_jj_WWHP=/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_WWHP.txt "
        binWWLP="CMS_jj_WWLP=/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_WWLP.txt "
     
        binZZHPB="CMS_jj_ZZHPB=/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_ZZHPBtagged.txt "
        binZZLPB="CMS_jj_ZZLPB=/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_ZZLPBtagged.txt "
        binWZHPB="CMS_jj_WZHPB=/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_WZHPBtagged.txt "
        binWZLPB="CMS_jj_WZLPB=/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_WZLPBtagged.txt "
        binWWHPB="CMS_jj_WWHPB=/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_WWHPBtagged.txt "
        binWWLPB="CMS_jj_WWLPB=/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_WWLPBtagged.txt "



        bin56="/usr/users/vscheurer/DijetCombineLimitCode/datacards/"+postfix+"CMS_jj_"+chan+"_"+str(mass)+"_13TeV_CMS_jj_comb_bb.txt "
        # comb01 = "combineCards.py " + bin0 + bin1 + " >" + bin01
        # comb34 = "combineCards.py " + bin3 + bin4 + " >" + bin34
        
        #comb567 = "combineCards.py " + bin5 + bin6 + bin7 + " >" + bin567
        #comb8910 = "combineCards.py " + bin8 + bin9 + bin10 + " >" + bin8910
        #comb5678910 = "combineCards.py " + bin5 + bin6 + bin7+ bin8 + bin9 + bin10 +  " >" + bin5678910
        
        #comb1112 = "combineCards.py " + bin11 + bin12 + " >" + bin1112
        #comb1314 = "combineCards.py " + bin13 + bin14 + " >" + bin1314
        #comb11121314 = "combineCards.py " + bin11 + bin12 + bin13 + bin14 + " >" + bin11121314
        
        combbb = "combineCards.py " + binZZHP +  binZZLP + binWZHP + binWZLP + binWWHP + binWWLP + binZZHPB +  binZZLPB + binWZHPB + binWZLPB + binWWHPB + binWWLPB +" >" + bin56
        #combbb = "combineCards.py " + binZZHP +  binZZLP + binWZHP + binWZLP + binWWHP + binWWLP + " >" + bin56

        if "q" in chan:
            # print comb34
            # os.system( comb34  )
            print comb1112
            os.system( comb1112  )
            print comb1314
            os.system( comb1314  )
            print comb11121314
            os.system( comb11121314  )
            
        else:
          # print comb01
          # os.system( comb01  )
          print combbb
          os.system( combbb  )
          #print comb8910
          #os.system( comb8910  )
          #print comb5678910
          #os.system( comb5678910  )
