# import os,commands, os.path
# import sys
# from ROOT import *
#
# node=[for i in range(10,40)]
#
#
# cmd = "qrsh -q debug.q -l hostname=t3wn40 find /scratch/$USER/ -user $USER -exec rm -rf {} \
# t3wn40"
# print cmd
# os.system(cmd)


import os

wn = []
# wn=[for i in range(10,40)]
for i in range(10,41):
  if i == 36 or i == 43 or i == 41: continue
  wn.append(i)
  
for w in wn:
 cmd = "qrsh -q debug.q -l hostname=t3wn%i find /scratch/$USER/ -user $USER -exec rm -rf {} \\" %w
 print cmd
 os.system(cmd)