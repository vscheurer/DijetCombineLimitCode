import os

masses=[1000,1200,1400,1600,1800,2000,2500,3000,3500,4000,4500]

for mass in masses:
  outputname = "submit_Wprime_"+str(mass)+".src"
  logname = "submit_Wprime_"+str(mass)+".log"
  outputfile = open(outputname,'w')
  outputfile.write('#!/bin/bash\n')
  outputfile.write("cd ${CMSSW_BASE}/src/DijetCombineLimitCode; eval `scramv1 run -sh`\n")
  outputfile.write("root -b -q 'R2JJFitter13TeV.cc("+str(mass)+","+'""'+",2)'\n")
  outputfile.close()
  
  command="rm "+logname
  print command
  os.system(command)
  #command="bsub -q 1nh -o "+logname+" source "+outputname
  command="chmod 755 ./"+outputname+";./"+outputname
  print command
  os.system(command)
  
masses =[m*100 for m in range(10,40+1)]
masses = []

for mass in masses:
  outputname = "submit_RS1VV_"+str(mass)+".src"
  logname = "submit_RS1VV_"+str(mass)+".log"
  outputfile = open(outputname,'w')
  outputfile.write('#!/bin/bash\n')
  outputfile.write("cd ${CMSSW_BASE}/src/DijetCombineLimitCode; eval `scramv1 run -sh`\n")
  outputfile.write("root -b -q 'R2JJFitter13TeV.cc("+str(mass)+","+'""'+",0)'\n")
  outputfile.close()
  
  command="rm "+logname
  print command
  os.system(command)
  #command="bsub -q 1nh -o "+logname+" source "+outputname
  command="chmod 755 ./"+outputname+";./"+outputname
  print command
  os.system(command)


masses =[m*100 for m in range(10,60+1)]
masses = []

for mass in masses:
  outputname = "submit_qV_"+str(mass)+".src"
  logname = "submit_qV_"+str(mass)+".log"
  outputfile = open(outputname,'w')
  outputfile.write('#!/bin/bash\n')
  outputfile.write("cd ${CMSSW_BASE}/src/DijetCombineLimitCode; eval `scramv1 run -sh`\n")
  outputfile.write("root -b -q 'R2JJFitter13TeV.cc("+str(mass)+","+'""'+",1)'\n")
  outputfile.close()
  
  command="rm "+logname
  print command
  os.system(command)
  #command="bsub -q 1nh -o "+logname+" source "+outputname
  command="chmod 755 ./"+outputname+";./"+outputname
  print command
  os.system(command)

masses =[m*100 for m in range(10,40+1)]
masses = []

for mass in masses:
  outputname = "submit_BulkVV_"+str(mass)+".src"
  logname = "submit_BulkVV_"+str(mass)+".log"
  outputfile = open(outputname,'w')
  outputfile.write('#!/bin/bash\n')
  outputfile.write("cd ${CMSSW_BASE}/src/DijetCombineLimitCode; eval `scramv1 run -sh`\n")
  outputfile.write("root -b -q 'R2JJFitter13TeV.cc("+str(mass)+","+'""'+",5)'\n")
  outputfile.close()
  
  command="rm "+logname
  print command
  os.system(command)
  #command="bsub -q 1nh -o "+logname+" source "+outputname
  command="chmod 755 ./"+outputname+";./"+outputname
  print command
  os.system(command)



  