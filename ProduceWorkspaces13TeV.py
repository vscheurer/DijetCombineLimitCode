import os,sys,time

masses =[m*100 for m in range(14,40+1)]#45+1)]
# masses =[m*100 for m in range(12,62+1)]
samples = [4]  # 0==RS1WW/ZZ 2==W'/Z' 4==BulkG WW/ZZ 6==qW/qZ
altfunc = '\"alt\"'
channel = 4
for sample in samples:
  for mass in masses:
    command = 'qsub -q short.q submitWorkspaces.sh %s %s %s'%(mass,sample,channel)
    print command
    os.system(command)


  
