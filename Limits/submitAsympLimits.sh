 #!/bin/bash
FULLTOYS=$1
CHANNEL=$2
MASSPOINT=$3
BIN=$4
POINT=$5
RMIN=$6
RMAX=$7
POSTFIX=$8

JOBLOGFILES="myout.txt myerr.txt"
OUTFILES="higgsCombine${CHANNEL}${BIN}.Asymptotic.mH${MASSPOINT}.root mlfit${CHANNEL}${BIN}${MASSPOINT}.root"
DBG=2
SEUSERSUBDIR=""
SEOUTFILES=""
HN_NAME=`whoami`
USER_SRM_HOME="srm://t3se01.psi.ch:8443/srm/managerv2?SFN=/pnfs/psi.ch/cms/trivcat/store/user"
TOPWORKDIR=/scratch/`whoami`
JOBDIR="limitBatchProducer_${MASSPOINT}${BIN}${CHANNEL}/"
CMSSW_DIR="/mnt/t3nfs01/data01/shome/thaarres/EXOVVAnalysisRunII/LimitCode/CMSSW_7_1_5"
BASEDIR=`pwd`
LOGDIRNAME=${BASEDIR}/limitBatchProducer
OUTDIR=${BASEDIR}/limitBatchProducer

# mkdir ${OUTDIR}
############ BATCH QUEUE DIRECTIVES ##############################
# Lines beginning with #$ are used to set options for the SGE
# queueing system (same as specifying these options to the qsub
# command

# Job name (defines name seen in monitoring by qstat and the
#     job script's stderr/stdout names)
#$ -N asympLimit_job

### Specify the queue on which to run
#$ -q short.q

# Change to the current working directory from which the job got
# submitted (will also result in the job report stdout/stderr being
# written to this directory)
#$ -cwd

# here you could change location of the job report stdout/stderr files
#  if you did not want them in the submission directory
#$ -o /mnt/t3nfs01/data01/shome/thaarres/EXOVVAnalysisRunII/LimitCode/CMSSW_7_1_5/src/DijetCombineLimitCode/limitBatchProducer/
#$ -e /mnt/t3nfs01/data01/shome/thaarres/EXOVVAnalysisRunII/LimitCode/CMSSW_7_1_5/src/DijetCombineLimitCode/limitBatchProducer/
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
source $VO_CMS_SW_DIR/cmsset_default.sh

cd $CMSSW_DIR/src
eval `scramv1 runtime -sh`
if test $? -ne 0; then
   echo "ERROR: Failed to source scram environment" >&2
   exit 1
fi

cd $WORKDIR
echo $RMIN
echo $RMAX
echo "CALCULATE ASYMPTOTIC LIMITS!">&2
echo "${BASEDIR}/datacards/${POSTFIX}CMS_jj_${CHANNEL}_${MASSPOINT}_13TeV_${BIN}.txt -M Asymptotic -v2 -m ${MASSPOINT} -n ${CHANNEL}${BIN} --rMax ${RMAX} --rMin ${RMIN}" >> myout.txt 2>>myerr.txt
echo "mv ${BASEDIR}/higgsCombine${CHANNEL}${BIN}.Asymptotic.mH${MASSPOINT}.root ${BASEDIR}/Limits/CMS_jj_${MASSPOINT}_${CHANNEL}_13TeV_${BIN}_asymptoticCLs_new.root" >> myout.txt 2>>myerr.txt
combine ${BASEDIR}/datacards/${POSTFIX}CMS_jj_${CHANNEL}_${MASSPOINT}_13TeV_${BIN}.txt -M Asymptotic -v2 -m ${MASSPOINT} -n ${CHANNEL}${BIN} --rMax ${RMAX} --rMin ${RMIN} >> myout.txt 2>>myerr.txt

echo "CALCULATE AND DRAW MAXLIKELIHOODFIT!">&2
echo "combine ${BASEDIR}/datacards/${POSTFIX}CMS_jj_${CHANNEL}_${MASSPOINT}_13TeV_${BIN}.txt -M MaxLikelihoodFit -m ${MASSPOINT} -n ${CHANNEL}${BIN}${MASSPOINT} --plots --saveShapes --rMax ${RMAX} --rMin ${RMIN}" >> myout.txt 2>>myerr.txt
echo "mv ${BASEDIR}/mlfit${CHANNEL}${BIN}${MASSPOINT}.root ${BASEDIR}/Limits/mlfits/mlfit${CHANNEL}${BIN}${MASSPOINT}.root" >> myout.txt 2>>myerr.txt
combine ${BASEDIR}/datacards/${POSTFIX}CMS_jj_${CHANNEL}_${MASSPOINT}_13TeV_${BIN}.txt -M MaxLikelihoodFit -m ${MASSPOINT} -n ${CHANNEL}${BIN}${MASSPOINT} --plots --saveShapes --rMax ${RMAX} --rMin ${RMIN} >> myout.txt 2>>myerr.txt

#   exit;
# fi




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
mv asympLimit_job.o$JOB_ID asympLimit_job.e$JOB_ID $LOGDIRNAME/.
mv ${OUTDIR}/higgsCombine${CHANNEL}${BIN}.Asymptotic.mH${MASSPOINT}.root ${BASEDIR}/Limits/${POSTFIX}CMS_jj_${MASSPOINT}_${CHANNEL}_13TeV_${BIN}_asymptoticCLs.root >> $LOGDIRNAME/asympLimit_job.o$JOB_ID
mv ${BASEDIR}/limitBatchProducer/mlfit${CHANNEL}${BIN}${MASSPOINT}.root ${BASEDIR}/Limits/mlfits/mlfit${CHANNEL}${BIN}${MASSPOINT}.root >> $LOGDIRNAME/asympLimit_job.o$JOB_ID
mv ${BASEDIR}/limitBatchProducer/${JOBDIR}/myout.txt  ${BASEDIR}/Limits/mlfits/mlfit${CHANNEL}${BIN}${MASSPOINT}.txt >> $LOGDIRNAME/asympLimit_job.o$JOB_ID
exit 0
