import os,sys
postfix = "sideband_"
# masses =[m*100 for m in range(12,40+1)]
masses =[1000,1200,1400,1800,2000,2500,3000,3500,4000]
# masses = [2000,3000]
samples = [2]  # 0==RS1WW/ZZ 2==W'/Z' 4==BulkG WW/ZZ else==qW/qZ
for mass in masses:
  for sample in samples:
    channel = 1    # 1==VV 2==qV 3==No purity
    if sample != 0 and sample != 2 and sample != 4: 
      channel = 2
    command = 'qsub -q short.q submitWorkspaceJobs.sh %i %i %i %s'%(mass,sample,channel,postfix)
    print command
    os.system(command)




  