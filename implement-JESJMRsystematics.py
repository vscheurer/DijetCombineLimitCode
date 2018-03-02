from ROOT import *
import fileinput

path = "/usr/users/dschaefer/SFrame_setup/ExoDiBosonAnalysis/forSystematics/"
prefixDCin = "datacards/CMS_jj_"
prefixDCout = "datacards/CMS_jj_"

prefix = "EXOVVSystematics/dijet"

purities = ["LP","HP","LPBtagged","HPBtagged"]
channels = ["WW","WZ","ZZ"]
#signals=["BulkWW","BulkZZ","WZ","ZprimeWW"]
signals = ["BulkZZ"]
masses_interpolated =[m*100 for m in range(12,40+1)]
massesInSystematics = [1200,1400,1600,1800,2000,2500,3000,3500,4000,4500]

#
# signals=["qW","qZ"]
# channels = ["qW","qZ"]
# masses_interpolated =[m*100 for m in range(12,62+1)]
# massesInSystematics = [1200,1400,1600,1800,2000,2500,3000,3500,4000,4500,5000,5500,6000,6500]

# purities = ["HP"]
# signals=["qZ"]
# channels = ["qZ"]
# masses_interpolated =[1200]
# massesInSystematics = [1200]


for purity in purities:
  ii = -1
  for signal in signals:
      ii += 1
      for ch in channels:
        chnl = ch + "_"
        if purity == "HPBtagged" or purity == "HP":
            testPurity = "HP"
        if purity == "LPBtagged" or purity == "LP":
            testPurity = "LP"
        if signal.find("BulkWW")  !=-1: label = "BulkWW"
        if signal.find("BulkZZ")  !=-1: label = "BulkZZ"
        if signal.find("WZ")      !=-1: label = "WprimeWZ"
        if signal.find("ZprimeWW")!=-1: label = "ZprimeWW"
        if signal.find("qW") !=-1: label = "QstarQW"
        if signal.find("qZ") !=-1: label = "QstarQZ"
        fname_JMS = path+"/JMS/JMSsys_"+testPurity+chnl+label+".txt"
        fname_JMR = path+"/JMR/JMRsys_"+testPurity+chnl+label+".txt"
        fname_JES = path+"/JES/JESsys_"+testPurity+chnl+label+".txt"
        fname_JER = path+"/JER/JERsys_"+testPurity+chnl+label+".txt"
        JMSUP={}
        JMSDOWN={}
        JMRUP={}
        JMRDOWN={}
        jesUP={}
        jesDOWN={}
        jerUP={}
        jerDOWN={}
        print "For %s_%s_%s :" %(signal,purity,ch)       
        print "Input systematics cards:"
        print fname_JMS
        print fname_JMR
        print fname_JES
        print fname_JER
        
        with open(fname_JMS,"r") as JMS:
          for l in JMS:
            if l.find('mass') != -1: continue
            for m in masses_interpolated:
              if not l.find("%i"%m) != -1: continue
              JMSup = 1 + float(l.split(' ')[1])/100. 
              JMSdown = 1 + float(l.split(' ')[2])/100.
              JMSUP[m] = JMSup
              JMSDOWN[m] = JMSdown
              
        with open(fname_JMR,"r") as JMR:
          for l in JMR:
            if l.find('mass') != -1: continue
            for m in masses_interpolated:
              if not l.find("%i"%m) != -1: continue
              jmrup = 1 + float(l.split(' ')[1])/100.
              jmrdown =1 + float(l.split(' ')[2])/100.
              JMRUP[m] = jmrup
              JMRDOWN[m] = jmrdown      
        
        with open(fname_JES,"r") as JES:
          for l in JES:
            if l.find('mass') != -1: continue
            for m in masses_interpolated:
              if not l.find("%i"%m) != -1: continue
              jesup = 1 + float(l.split(' ')[1])/100. 
              jesdown = 1 + float(l.split(' ')[2])/100.
              jesUP[m] = jesup
              jesDOWN[m] = jesdown
            
        with open(fname_JER,"r") as JER:
          for l in JER:
            if l.find('mass') != -1: continue
            for m in masses_interpolated:
              if not l.find("%i"%m) != -1: continue
              jerup = 1 + float(l.split(' ')[1])/100.
              jerdown =1 + float(l.split(' ')[2])/100.
              jerUP[m] = jerup
              jerDOWN[m] = jerdown
        
        for m in masses_interpolated:
          if not m in massesInSystematics:
            print "THIS MASSPOINT IS NOT IN LIST!! Extrapolating to lower: "
            ref = m - 100
            JMSUP[m] = JMSUP[ref]
            JMSDOWN[m] = JMSDOWN[ref]
            JMRUP[m]   = JMRUP[ref]
            JMRDOWN[m] = JMRDOWN[ref]
            jesUP[m]   = jesUP[ref]
            jesDOWN[m] = jesDOWN[ref]
            jerUP[m]   = jerUP[ref]
            jerDOWN[m] = jerDOWN[ref]
            print " JMSUP[%i]=JMSUP[%i]=%f" %(m,ref,JMSUP[m])
            
          
        for m in masses_interpolated:
          print ""
          print "Mass = %i: " %(m)
          print " JMS UP/DOWN =    %.4f     %.4f  " %(JMSUP[m],JMSDOWN[m])
          print " JMR UP/DOWN =    %.4f     %.4f  " %(JMRUP[m],JMRDOWN[m])
          print " JES UP/DOWN =    %.4f     %.4f  " %(jesUP[m],jesDOWN[m])
          print " JER UP/DOWN =    %.4f     %.4f  " %( jerUP[m],jerDOWN[m])
          fname_datacard_in = prefixDCin + "%s_%i"%(signal,m)+"_13TeV_CMS_jj_"+ch+purity+".txt"
          fname_datacard_out = prefixDCout + "%s_%i"%(signal,m)+"_13TeV_CMS_jj_"+ch+purity+".txt"
          print "Input datacard:  %s" %fname_datacard_in
          print "Output datacard: %s"  %fname_datacard_out
          lines = []
          try:
            with open(fname_datacard_in) as infile:
              for line in infile:
                if not (line.find("CMS_mass_scale_j_13TeV")!=-1 or line.find("CMS_mass_res_j_13TeV")!=-1 or line.find("CMS_scale_j_13TeV")!=-1 or line.find("CMS_res_j_13TeV")!=-1 ):
                  lines.append(line)
              jms="\nCMS_mass_scale_j_13TeV       lnN      %s/%s      %s/%s      -                          \n"%(JMSUP[m],JMSDOWN[m],JMSUP[m],JMSDOWN[m])
              jmr="CMS_mass_res_j_13TeV         lnN      %s/%s      %s/%s      -                          \n"%(JMRUP[m],JMRDOWN[m],JMRUP[m],JMRDOWN[m])
              jes="CMS_scale_j_13TeV            lnN      %s/%s      %s/%s      -  # jet energy scale      \n"%(jesUP[m],jesDOWN[m],jesUP[m],jesDOWN[m])
              jer="CMS_res_j_13TeV              lnN      %s/%s      %s/%s      -  # jet energy resolution \n"%(jerUP[m],jerDOWN[m],jerUP[m],jerDOWN[m])
              lines.append(jms)
              lines.append(jmr)
              lines.append(jes)
              lines.append(jer)

              with open(fname_datacard_out, 'w') as outfile:
                for line in lines:
                  outfile.write(line)
              print "PRINTED TO: %s"  %fname_datacard_out
          except EnvironmentError: # parent of IOError, OSError *and* WindowsError where available
              print 'oops, datacard not found!'