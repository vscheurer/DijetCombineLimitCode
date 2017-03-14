#!/bin/bash

DATACARD=$1
MASS=$2
OUTFILENAME=$3
BKG=$4
SIGWS=$5

JOBLOGFILES="myout.txt myerr.txt"
# OUTFILES=""
DBG=2
SEUSERSUBDIR=""
SEOUTFILES=""
HN_NAME=`whoami`
USER_SRM_HOME="srm://t3se01.psi.ch:8443/srm/managerv2?SFN=/pnfs/psi.ch/cms/trivcat/store/user"
TOPWORKDIR=/scratch/`whoami`

JOBDIR=submitlimits-$JOB_ID
#CMSSW_DIR="/mnt/t3nfs01/data01/shome/thaarres/EXOVVAnalysisRunII/LimitCode/CMSSW_7_1_5"
CMSSW_DIR="/mnt/t3nfs01/data01/shome/dschafer/CMSSW_7_4_7"
BASEDIR=`pwd`
LOGDIRNAME=${BASEDIR}/${JOBDIR}
OUTDIR=${BASEDIR}/${JOBDIR}
#mkdir ${OUTDIR}
############ BATCH QUEUE DIRECTIVES ##############################
# Lines beginning with #$ are used to set options for the SGE
# queueing system (same as specifying these options to the qsub
# command

# Job name (defines name seen in monitoring by qstat and the
#     job script's stderr/stdout names)
#$ -N interpolateAll_job

### Specify the queue on which to run
#$ -q short.q

# Change to the current working directory from which the job got
# submitted (will also result in the job report stdout/stderr being
# written to this directory)
#$ -cwd

# here you could change location of the job report stdout/stderr files
#  if you did not want them in the submission directory
#$ -o $OUTDIR
#$ -e $OUTDIR
#################################################################

##### MONITORING/DEBUG INFORMATION ###############################
DATE_START=`date +%s`
echo "Job started at " `date`
cat <<EOF
################################################################
## QUEUEING SYSTEM SETTINGS:
HOME=$HOME
USER=$USER
JOB_ID=$JOB_ID
JOB_NAME=$JOB_NAME
HOSTNAME=$HOSTNAME
TASK_ID=$TASK_ID
QUEUE=$QUEUE

EOF

echo "################################################################"

if test 0"$DBG" -gt 0; then
   echo "######## Environment Variables ##########"
   env
   echo "################################################################"
fi


##### SET UP WORKDIR AND ENVIRONMENT ######################################
STARTDIR=`pwd`
WORKDIR=$TOPWORKDIR/$JOBDIR
RESULTDIR=$STARTDIR/$JOBDIR
if test x"$SEUSERSUBDIR" = x; then
   SERESULTDIR=$USER_SRM_HOME/$HN_NAME/$JOBDIR
else
   SERESULTDIR=$USER_SRM_HOME/$HN_NAME/$SEUSERSUBDIR
fi
if test -e "$WORKDIR"; then
   echo "ERROR: WORKDIR ($WORKDIR) already exists! Aborting..." >&2
   exit 1
fi
mkdir -p $WORKDIR
if test ! -d "$WORKDIR"; then
   echo "ERROR: Failed to create workdir ($WORKDIR)! Aborting..." >&2
   exit 1
fi

cd $WORKDIR
cat <<EOF
################################################################
## JOB SETTINGS:
STARTDIR=$STARTDIR
WORKDIR=$WORKDIR
RESULTDIR=$RESULTDIR
SERESULTDIR=$SERESULTDIR
EOF

###########################################################################
## YOUR FUNCTIONALITY CODE GOES HERE
# set up CMS environment

# do to: write submission script like this :
#qsub -q all.q submitLimits.sh CMS_jj_qZ_2000_13TeV_CMS_jj_qZHP 2000 CMS_jj_2000_qZ_13TeV_CMS_jj_qZHP_asymptoticCLs_new.root CMS_jj_bkg_qV_13TeV CMS_jj_qZ_2000_13TeV

#RUNSCRIPT=${BASEDIR}/combine
INFILE=${BASEDIR}/${INPUT}

source $VO_CMS_SW_DIR/cmsset_default.sh

cd $CMSSW_DIR/src
eval `scramv1 runtime -sh`
if test $? -ne 0; then
   echo "ERROR: Failed to source scram environment" >&2
   exit 1
fi

cd $WORKDIR
# copy needed files:                                                                                          
mkdir datacards                                                                                         
mkdir workspaces                                                                                        
cp /mnt/t3nfs01/data01/shome/dschafer/CMSSW_7_4_7/src/DijetCombineLimitCode/datacards/${DATACARD}.txt datacards/  
cp /mnt/t3nfs01/data01/shome/dschafer/CMSSW_7_4_7/src/DijetCombineLimitCode/workspaces/${SIGWS}.root workspaces/
cp /mnt/t3nfs01/data01/shome/dschafer/CMSSW_7_4_7/src/DijetCombineLimitCode/workspaces/${BKG}.root workspaces/     
                                                                                       
cd datacards                                                                                 
                                                                                       
    
                                                                                       
                                                                                       
                                                                                       
#echo "input: ${INPUT}" >> myout.txt 2>>myerr.txt                                                             
echo "datacard: ${DATACARD}" >> myout.txt 2>>myerr.txt
#combine datacards/CMS_jj_altBulkWW_2000_13TeV_CMS_jj_WWHP.txt -M Asymptotic -v2 -m 2000 -n BulkWW_M2000_alt --rMax 100 --rMin 0.000001
echo "combine ${DATACARD}.txt -M Asymptotic -m ${MASS} -n test --rMax 100 --rMin 0.000001" >> myout.txt 2>>myerr.txt
echo "mv  higgsCombineTest.Asymptotic.mH${MASS}.root ${OUTFILENAME}" >> myout.txt 2>>myerr.txt
combine ${DATACARD}.txt -M Asymptotic -m ${MASS} -n test --rMax 100 --rMin 0.000001 >> myout.txt 2>>myerr.txt
mv  higgs*.root /mnt/t3nfs01/data01/shome/dschafer/CMSSW_7_4_7/src/DijetCombineLimitCode/test/${OUTFILENAME} >> myout.txt 2>>myerr.txt
mv myerr.txt ../
mv myout.txt ../

#### RETRIEVAL OF OUTPUT FILES AND CLEANING UP ############################
cd $WORKDIR
if test 0"$DBG" -gt 0; then
    echo "########################################################"
    echo "############# Working directory contents ###############"
    echo "pwd: " `pwd`
    ls -Rl
    echo "########################################################"
    echo "YOUR OUTPUT WILL BE MOVED TO $RESULTDIR"
    echo "########################################################"
fi

if test x"$JOBLOGFILES" != x; then
   mkdir -p $RESULTDIR
   if test ! -e "$RESULTDIR"; then
          echo "ERROR: Failed to create $RESULTDIR ...Aborting..." >&2
          exit 1
   fi
   for n in $JOBLOGFILES; do
       if test ! -e $WORKDIR/$n; then
          echo "WARNING: Cannot find output file $WORKDIR/$n. Ignoring it" >&2
       else
          cp -a $WORKDIR/$n $RESULTDIR/$n
          if test $? -ne 0; then
             echo "ERROR: Failed to copy $WORKDIR/$n to $RESULTDIR/$n" >&2
          fi
   fi
   done
fi

if test x"$OUTFILES" != x; then
   if test 0"$DBG" -ge 2; then
      srmdebug="-v"
   fi
   for n in $OUTFILES; do
       if test ! -e $WORKDIR/$n; then
          echo "WARNING: Cannot find output file $WORKDIR/$n. Ignoring it" >&2
       else
	  cp -a $WORKDIR/$n $OUTDIR/$n
          if test $? -ne 0; then
             echo "ERROR: Failed to copy $WORKDIR/$n to $OUTDIR/$n" >&2
          fi
   fi
   done
fi

echo "Cleaning up $WORKDIR"
rm -rf $WORKDIR

rm -rf $LOGDIRNAME/$JOBDIR
mv $STARTDIR/$JOBDIR $LOGDIRNAME/.

###########################################################################
DATE_END=`date +%s`
RUNTIME=$((DATE_END-DATE_START))
echo "################################################################"
echo "Job finished at " `date`
echo "Wallclock running time: $RUNTIME s"

cd ${BASEDIR}
mv interpolateAll_job.o$JOB_ID interpolateAll_job.e$JOB_ID $LOGDIRNAME/.
exit 0
