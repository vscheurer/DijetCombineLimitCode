#!/usr/bin/python
from optparse import OptionParser
import os,commands, os.path
import sys
from ROOT import *
import math
import time
import copy
from array import *
import CMS_lumi, tdrstyle
# from ROOT import Math/QuantFuncMathCore


#Remember to have a version of CMS_lumi.C/h and tdrstyle.C/py in your folder!!

tdrstyle.setTDRStyle()
gStyle.SetOptFit(1) 
CMS_lumi.lumi_13TeV = "1.26 fb^{-1}"
CMS_lumi.writeExtraText = 1
CMS_lumi.extraText = "Simulation"
CMS_lumi.lumi_sqrtS = "13 TeV" # used with iPeriod = 0, e.g. for simulation-only plots (default is an empty string)
iPos = 11
if( iPos==0 ): CMS_lumi.relPosX = 0.12
iPeriod = 0


sqrtS = 13000.
lumi = 1263.89

def performFit(fInputFile, fPlot, fNbins, fBins,fFitXmin, fFitXmax,fLabel,  fOutputFile):
  
  fits = []
  residuals = []
  
  xbins = array('d',fBins)
  
# --------------------------------------- Rebinning-->hMass_rebinned ---------------------------------------
  file =TFile(fInputFile,"READ")
  hMass = file.Get(fPlot)
  # hMass.Scale(1/lumi)
  # hMass.SetBinErrorOption(TH1.kPoisson)

  hMass_rebinned = hMass.Rebin(fNbins,"hMass_rebinned",xbins)
  

  hMassNEW = TH1F("hist_mass","",fNbins,xbins)

# --------------------------------------- Filling new h dividing by binwidth ---------------------------------------
  for i in range (1,fNbins):
    bincontent = hMass_rebinned.GetBinContent(i)
    binwidth = hMass_rebinned.GetBinWidth(i)
    hMassNEW.SetBinContent(i,bincontent/(binwidth))
    # hMassNEW.SetBinError(i,(hMass_rebinned.GetBinError(i)/(binwidth)))


# ---------------------------- Create graph from data with Poisson error bars (for data) ----------------------------    
  x=[]
  y=[]
  exl=[]
  exh=[]
  eyl=[]
  eyh=[]
  alpha = 1 - 0.6827
  for i in range(0,fNbins):
    n    = hMass_rebinned.GetBinContent(i+1)
    dm   = hMass_rebinned.GetBinWidth(i+1)
    mass = hMass_rebinned.GetBinCenter(i+1)
    xl   = hMass_rebinned.GetBinLowEdge(i+1)
    xh   = xl+dm
    x.append( (xl+xh)/2.)
    exl.append( dm/2.)
    exh.append( dm/2.)
    y.append( n / (dm))
    # l = 0.5*TMath.ChisquareQuantile(alpha/2,2*n) #as used in dijet analysis
    # h = 0.5*TMath.ChisquareQuantile(1-alpha/2,2*(n+1))
    if(n==0): l = 0
    else: l = Math.gamma_quantile(alpha/2,n,1.) #as recommended in https://twiki.cern.ch/twiki/bin/view/CMS/PoissonErrorBars
    h = Math.gamma_quantile_c(alpha/2,n+1,1) 
    if (fInputFile.find("DATA")!=-1):
      eyl.append( (n-l)/(dm) )
      eyh.append( (h-n)/(dm) )
    if (fInputFile.find("QCD")!=-1):
      eyl.append(hMass_rebinned.GetBinError(i+1)/dm)
      eyh.append(hMass_rebinned.GetBinError(i+1)/dm )
    #print "%f   %f    %f    %f    %f     %f" % (x[i],y[i],exl[i],exh[i],eyl[i],eyh[i])
  
  vx = array("f",x)
  vy = array("f",y)
  vexl = array("f",exl)
  vexh = array("f",exh)
  veyl = array("f",eyl)
  veyh = array("f",eyh)
  
  #data in TGraph format
  g = TGraphAsymmErrors(fNbins,vx,vy,vexl,vexh,veyl,veyh)
  g.SetName("g_data")
 

  nBins_fit1 = hMassNEW.FindBin(fFitXmax)- hMassNEW.FindBin(fFitXmin)
  
  zeroBins = 0
  for i in range(0,nBins_fit1):
      if hMassNEW.GetBinContent(hMassNEW.FindBin(fFitXmin)+i)==0: 
        zeroBins +=1        
  nBins_fit = nBins_fit1 - zeroBins
  
  rss = []
  chi2 = []
  dof = []
  fisher = []
  ConfidenceLevel = []
  fpdfs = []
  
  #For alternate F-test inspired by Yongjie (not skipping empty bins)
  Altfisher = []
  AltConfidenceLevel = []
  rssALL = []
  chi2ALL = []
  dofALL = []
  
# --------------------------------------- Do fits ---------------------------------------
  f = 0
  FunctionTypes = [-2,0,1,2,3]
  for FunctionType in FunctionTypes:
    print "######################################################################################################################################"
    print "########################################  Do fit for function type %i :###############################################################" %FunctionType
    print "######################################################################################################################################"
    fitresult = doFit(FunctionType,hMassNEW,hMass,g,fFitXmin,fFitXmax,fNbins,xbins)
    M1Bkg = fitresult[3]
    hist_fit_residual_vsMass = fitresult[4]
    nPar = nBins_fit - fitresult[1] - 1
    # if FunctionType != -2:fits.append(M1Bkg)
    # if FunctionType != -2:residuals.append(hist_fit_residual_vsMass) \\if skipping 2 paramter fit
    fits.append(M1Bkg)
    residuals.append(hist_fit_residual_vsMass)
    rss.append(fitresult[0])
    chi2.append(fitresult[2])
    dof.append(fitresult[1])
    rssALL.append(fitresult[7])
    chi2ALL.append(fitresult[6])
    dofALL.append(fitresult[5])
    print "chi2 / dof for f%d = %f / %d" % (FunctionType,fitresult[2],fitresult[1])
    if (f>0):
      result = FisherTest(rss[f-1],rss[f],dof[f-1],dof[f],nBins_fit)
      if (f>3):
        print f
        result = FisherTest(rss[f-3],rss[f],dof[f-3],dof[f],nBins_fit) #now comparing alternate four paramter fit to 3 paramter fit
      F = result[0]
      CL = result[1]
      fisher.append(F)
      ConfidenceLevel.append(CL)
      fpdfs.append(result[2])
      Altresult = AltFisherTest(rssALL[f-1],rss[f],dofALL[f-1],dofALL[f],nBins_fit1) #alternative Fisher test using TMath.FDistI
      if (f>3):
        print f
        Altresult = AltFisherTest(rssALL[f-3],rss[f],dofALL[f-3],dofALL[f],nBins_fit1)
      AltF = Altresult[0]
      AltCL = Altresult[1]
      Altfisher.append(AltF)
      AltConfidenceLevel.append(AltCL)
    f += 1
# --------------------------------------- Print some results ---------------------------------------  
  print "################################################################################################################################################"
  print ""
  print ""
  print "FOR CATEGORY: %s" %fLabel
  print ""
  print ""
  print "#############################################            ALTERNATIVE METHOD (from Yong)            #############################################"  
  print ""
  print "Summary:"   
  print "Residuals = "+str(rssALL)
  print "$\chi^2$ = "+str(chi2ALL)
  print "D.O.F ="+str(dofALL)
  print "Fishers F = "+str(Altfisher)
  print "ConfidenceLevel = "+str(AltConfidenceLevel)  
  print "-----------------------------------------"
  print ""
  print "For Latex table:"
  print ""
  print "\multicolumn{4}{|c|}{%s}\\"%fLabel
  print "\hline"
  print "Function & Residuals & $\chi^2$ & ndof \\"
  print "\hline"
  print "2 par & %.3f & %.3f & %i \\"%(rssALL[0],chi2ALL[0],dofALL[0])
  print "3 par & %.3f & %.3f & %i \\"%(rssALL[1],chi2ALL[1],dofALL[1])
  print "4 par & %.3f & %.3f & %i \\"%(rssALL[2],chi2ALL[2],dofALL[2])
  print "5 par & %.3f & %.3f & %i \\"%(rssALL[3],chi2ALL[3],dofALL[3])
  print "Alt. 4 par & %.3f & %.3f & %i \\"%(rssALL[4],chi2ALL[4],dofALL[4])
  print "\hline"
  print "Fishers23 \multicolumn{2}{l}{%.3f}&CL \multicolumn{2}{l|}{%.3f}\\"%(Altfisher[0],AltConfidenceLevel[0])
  print "Fishers34 \multicolumn{2}{l}{%.3f}&CL \multicolumn{2}{l|}{%.3f}\\"%(Altfisher[1],AltConfidenceLevel[1])
  print "Fishers45 \multicolumn{2}{l}{%.3f}&CL \multicolumn{2}{l|}{%.3f}\\"%(Altfisher[2],AltConfidenceLevel[2])
  print "Fishers3Alt4 \multicolumn{2}{l}{%.3f}&CL \multicolumn{2}{l|}{%.3f}\\"%(Altfisher[3],AltConfidenceLevel[3])
  print "\hline"
  print ""
  print ""
  print ""
  print "#############################################            DIJET METHOD           #############################################"
  print ""
  print "Summary:" 
  print "Residuals = "+str(rss)
  print "$\chi^2$ = "+str(chi2)
  print "D.O.F ="+str(dof)
  print "Fishers F = "+str(fisher)
  print "ConfidenceLevel = "+str(ConfidenceLevel)
  print "Functions = "+str(FunctionTypes)
  print "Nr. bins in fit = "+str(nBins_fit)
  print "-----------------------------------------"
  print ""
  print "For Latex table:"
  print ""
  print "\multicolumn{4}{|c|}{%s}\\"%fLabel
  print "\hline"
  print "Function & Residuals & $\chi^2$ & ndof \\"
  print "\hline"
  print "2 par & %.3f & %.3f & %i \\"%(rss[0],chi2[0],dof[0])
  print "3 par & %.3f & %.3f & %i \\"%(rss[1],chi2[1],dof[1])
  print "4 par & %.3f & %.3f & %i \\"%(rss[2],chi2[2],dof[2])
  print "5 par & %.3f & %.3f & %i \\"%(rss[3],chi2[3],dof[3])
  print "Alt. 4 par& %.3f & %.3f & %i \\"%(rss[4],chi2[4],dof[4])
  print "\hline"
  print "Fishers23 \multicolumn{2}{l}{%.3f}&CL \multicolumn{2}{l|}{%.3f}\\"%(fisher[0],ConfidenceLevel[0])
  print "Fishers34 \multicolumn{2}{l}{%.3f}&CL \multicolumn{2}{l|}{%.3f}\\"%(fisher[1],ConfidenceLevel[1])
  print "Fishers45 \multicolumn{2}{l}{%.3f}&CL \multicolumn{2}{l|}{%.3f}\\"%(fisher[2],ConfidenceLevel[2])
  print "Fishers3Alt4 \multicolumn{2}{l}{%.3f}&CL \multicolumn{2}{l|}{%.3f}\\"%(fisher[3],ConfidenceLevel[3])
  print "\hline"

  print "##############################################################################################################"
  print "##############################################################################################################"
  alpha = 0.1
  print  " F - test results using alternate method: " 
  print "-----------------------------------------"
  if (AltConfidenceLevel[0]>alpha):
    print  "A linear fit is sufficient to describe these data" 
  elif (AltConfidenceLevel[1]>alpha):
    print  " A three parameter fit is sufficient to describe these data." 
  elif (AltConfidenceLevel[2]>alpha):
    print  " A four paramter fit appears adequate to describe these data." 
  elif (AltConfidenceLevel[3]>alpha):
    print  " The alternate fit is better than the 3 parameter default fit" 
  else:
    print  " A better fit is needed for these data. " 
    print  " You should check for higher-order polynomials. "
  print "-----------------------------------------"
  print ""
  print ""
  print ""
  print ""
  print ""
  print ""


# --------------------------------------- Make plots ---------------------------------------
    
  # DrawFit(hMassNEW,g,fits,residuals,FunctionType,nPar,fFitXmin,fFitXmax,fLabel,fOutputFile) #when final fit is decided, plots only one
  FitComparisons(hMassNEW,g,fits,residuals,FunctionType,nPar,fFitXmin,fFitXmax,fLabel,fOutputFile,chi2,dof) #draw all fit functions with residuals
  
  del file
  
  
  
  
# ------------------------------------------------------------------------------------------    
def doFit(FunctionType,hMassNEW,hMass,g,fFitXmin,fFitXmax,fNbins,xbins):
  
  if( FunctionType==-2 ):    
      nPar=2
      BKGfit = TF1("BKGfit%i"%FunctionType," [0] / ( TMath::Power(x/13000,[1]) )",fFitXmin,fFitXmax)
      BKGfit.SetParameter(0,1.59778e-08)
      BKGfit.SetParameter(1,7.78922e+00)

  if( FunctionType==0 ):
    print "Fitting three parameter default function!"
    nPar=3
    BKGfit = TF1("BKGfit%i"%FunctionType,"( [0]*TMath::Power(1-x/13000,[1]) ) / ( TMath::Power(x/13000,[2]) )",fFitXmin,fFitXmax)
    BKGfit.SetParameter(0,4.98767e-07)


  elif( FunctionType==1 ):
    print "Fitting four parameter default function!"
    nPar=4
    BKGfit = TF1("BKGfit%i"%FunctionType,"( [0]*TMath::Power(1-x/13000,[1]) ) / ( TMath::Power(x/13000,[2]+[3]*log(x/13000)) )",fFitXmin,fFitXmax)
    BKGfit.SetParameter(0,2.01354e-07)
    BKGfit.SetParameter(1,0.224623)
    # BKGfit.SetParameter(2,7.31499e+00)
    # BKGfit.SetParameter(3,8.63532e-02)
    
  elif( FunctionType==2 ):
    print "Fitting five parameter  function!"
    nPar=5
    # BKGfit =  TF1("BKGfit%i"%FunctionType,"( [0]*TMath::Power(1-x/13000,[1]) ) / ( TMath::Power(x/13000,[2]+[3]*log(x/13000)+[4]*TMath::Power(log(x/13000),2)) )",fFitXmin,fFitXmax)
    BKGfit =  TF1("BKGfit%i"%FunctionType,"( [0]*TMath::Power(1-x/13000,[1])*(1+[4]*x/13000) ) / ( TMath::Power(x/13000,[2]+[3]*log(x/13000)) )",fFitXmin,fFitXmax)
    BKGfit.SetParameter(0,4.98767e-07)

    #
  elif( FunctionType==3 ):
    print "Fitting four parameter alternate function!"
    nPar=4
    BKGfit =  TF1("BKGfit%i"%FunctionType,"( [0]*TMath::Power(1-x/13000 + [3]*TMath::Power(x/13000,2),[1]) ) / ( TMath::Power(x/13000,[2]) )",fFitXmin,fFitXmax)
    BKGfit.SetParameter(0,4.98767e-07)

    
  
  
  stopProgram=1;
  for loop in range (0,10):
    r = hMassNEW.Fit("BKGfit%i"%FunctionType,"SR","",fFitXmin,fFitXmax)          
    fitStatus = int(r)
    print "fit status : %d" % fitStatus
    if(fitStatus==0):
      stopProgram=0
      r.Print("V")  
      break
 
  if(stopProgram==1):
    print "######################" 
    print"######################" 
    print "ERROR : Fit failed!!!!" 
    print "######################" 
    print "######################"
  
  NumberOfVarBins = 0
  NumberOfObservations_VarBin = 0
  chi2_VarBin = 0.
  chi2_VarBin_ALL = 0.
  chi2_VarBin_notNorm = 0.
  chi2_VarBin_notNorm_ALL = 0.
  
  
  
  hist_fit_residual_vsMass =  TH1D("hist_fit_residual_vsMass","hist_fit_residual_vsMass",fNbins,xbins)
  
  for bin in range (1,hMassNEW.GetNbinsX()):
    if( hMassNEW.GetXaxis().GetBinLowEdge(bin)>=fFitXmin and hMassNEW.GetXaxis().GetBinUpEdge(bin)<=fFitXmax ):
       NumberOfVarBins += 1
       data = hMassNEW.GetBinContent(bin)
       err_data_low = g.GetErrorYlow(bin-1) 
       err_data_high= g.GetErrorYhigh(bin-1)
       fit = BKGfit.Integral(hMassNEW.GetXaxis().GetBinLowEdge(bin) , hMassNEW.GetXaxis().GetBinUpEdge(bin) )
       fit = fit / ( hMassNEW.GetBinWidth(bin) )
       # fit = BKGfit.Eval(hMassNEW.GetBinCenter(bin)) #yields same results
       if(fit > data):
         err_tot = err_data_high
       else:
         err_tot = err_data_low
       # if(err_tot==0):continue
       fit_residual = (data - fit) / err_tot
       err_fit_residual = 1
       chi2_VarBin_notNorm_ALL += pow( (data - fit) , 2 )
       chi2_VarBin_ALL += pow( (data - fit) , 2 ) / pow( err_tot , 2 )
       if (hMassNEW.GetBinContent(bin)>0):
         NumberOfObservations_VarBin+=1
         chi2_VarBin += pow( (data - fit) , 2 ) / pow( err_tot , 2 )	 
         chi2_VarBin_notNorm += pow( (data - fit) , 2 ) 	 

       hist_fit_residual_vsMass.SetBinContent(bin,fit_residual)
       hist_fit_residual_vsMass.SetBinError(bin,err_fit_residual)
  
  ndf_VarBin = NumberOfObservations_VarBin - nPar -1

  
  print "Chi2 Minuit = %f"%BKGfit.GetChisquare()
  print "NDF Minuit = %i"%BKGfit.GetNDF()

  
  return [chi2_VarBin_notNorm,ndf_VarBin,chi2_VarBin,BKGfit,hist_fit_residual_vsMass,nPar,chi2_VarBin_ALL,chi2_VarBin_notNorm_ALL]   

def FisherTest(RSS_1,RSS_2,dof_1,dof_2,N):
  RSS1 = RSS_1
  RSS2 = RSS_2
  n1 = N - dof_1 - 1
  n2 = N - dof_2 - 1
  # print "Residuals function 1 = %f" % RSS_1
  # print "Residuals function 2 = %f" % RSS_2
  # print "DOF function 1 = %i" % n1
  # print "DOF function 2 = %i" % n2
  # print "DOF TOT = %i" % N
  # print "n1 = %d    n2 = %d" % (n1,n2)
  # print "((RSS1-RSS2)/(n2-n1)) = %f" %((RSS1-RSS2)/(n2-n1))
  # print "(RSS2/(N-n2) = %f" %(RSS2/(N-n2))
  F = ((RSS1-RSS2)/(n2-n1)) / (RSS2/(N-n2))
  print "F = %f" % F
  F_dist = TF1("F_distr","TMath::Sqrt( (TMath::Power([0]*x,[0]) * TMath::Power([1],[1])) / (TMath::Power([0]*x + [1],[0]+[1])) ) / (x*TMath::Beta([0]/2,[1]/2))",0,1000)
  print "d1 = %d    d2 = %d" %(n2-n1,N-n2)
  F_dist.SetParameter(0, n2-n1)
  F_dist.SetParameter(1, N-n2)
  CL = 1 - F_distr.Integral(0.00000001,F)
  print "CL == %f" %CL
  return [F,CL,F_dist]

def AltFisherTest(RSS_0,RSS_1,dof_0,dof_1,N):
  p1_10 = dof_1-dof_0;
  p2_10 = N-dof_1;
  rss0 = RSS_0
  rss1 = RSS_1
  Ftest_10 = (rss0-rss1)/p1_10 / (rss1/p2_10)
  good_CL10 =  1.-TMath.FDistI(Ftest_10,p1_10,p2_10)
  print "Ftest10 = %f" %(1.-TMath.FDistI(Ftest_10,p1_10,p2_10))
  return [Ftest_10,good_CL10]
    
     
# ---------------------------------------------------------------------------------------------------------------------------
def DrawFit(hMassNEW,g,M1Bkg,hist_fit_residual_vsMass,FunctionType,nPar,fFitXmin,fFitXmax,fLabel,fOutputFile):
  W = 600
  H = 700
  H_ref = 700 
  W_ref = 600 
  T = 0.08*H_ref
  B = 0.12*H_ref
  L = 0.12*W_ref
  R = 0.04*W_ref

  c1 = TCanvas("c1","VV mass fit",W,H)
  c1.GetWindowHeight()
  c1.GetWindowWidth()
  c1.SetLogy()
  c1.Divide(1,2,0,0,0)

  #------------ pad 1  ----------------
  c1.cd(1)
  p11_1 = c1.GetPad(1)
  p11_1.SetPad(0.01,0.26,0.99,0.98)
  p11_1.SetLogy()
  p11_1.SetRightMargin(0.05)
  p11_1.SetTopMargin(0.05)
  p11_1.SetFillColor(0)
  p11_1.SetBorderMode(0)
  p11_1.SetFrameFillStyle(0)
  p11_1.SetFrameBorderMode(0)

  #Pave text
  #addInfo = TPaveText(0.1558691,0.30735043,0.3750171,0.4070085,"NDC")
  addInfo = TPaveText(0.2358691,0.04035043,0.5050171,0.1870085,"NDC")
  
  addInfo.AddText(fLabel)
  # addInfo.AddText("65 GeV < M_{P} < 105 GeV")
  # addInfo.AddText("|#eta| < 2.4, p_{T} > 200 GeV")
  addInfo.AddText("M_{jj} > 1 TeV, |#Delta#eta_{jj}| < 1.3")
  addInfo.SetFillColor(0)
  addInfo.SetLineColor(0)
  addInfo.SetFillStyle(0)
  addInfo.SetBorderSize(0)
  addInfo.SetTextFont(42)
  addInfo.SetTextSize(0.040)
  addInfo.SetTextAlign(12)
  
  vFrame = p11_1.DrawFrame(fFitXmin,0.000005,fFitXmax,hMassNEW.GetMaximum()*5.0)  
  vFrame.SetTitle("")
  vFrame.SetXTitle("M_{jj} [GeV]")
  vFrame.SetYTitle("#frac{dN}{dm_{jj}}")
  vFrame.GetXaxis().SetTitleSize(0.06)
  vFrame.GetXaxis().SetTitleOffset(0.95)
  vFrame.GetXaxis().SetLabelSize(0.05)
  vFrame.GetYaxis().SetTitleSize(0.06)
  #vFrame.GetYaxis().SetTitleOffset(1.0)
  vFrame.GetYaxis().SetLabelSize(0.05)

  g.SetMarkerSize(0.9)
  g.SetMarkerStyle(20)
  g.GetXaxis().SetNdivisions(405)
  g.Draw("pe0 same")
  M1Bkg[0].SetLineWidth(2)
  M1Bkg[0].SetLineStyle(1)
  M1Bkg[0].SetLineColor(kRed)
  M1Bkg[0].Draw("same")

 
  legend = TLegend(0.56164991,0.62,0.8503575,0.80)
  legend.SetTextSize(0.038)
  legend.SetLineColor(0)
  legend.SetShadowColor(0)
  legend.SetLineStyle(1)
  legend.SetLineWidth(1)
  legend.SetFillColor(0)
  legend.SetFillStyle(0)
  legend.SetMargin(0.35)
  legend.AddEntry(g,"CMS data","lpe")
  legend.AddEntry(M1Bkg[0], "Fit","l")
  # legend.AddEntry(M1Bkg[3], "Fit","l")
  legend.Draw("same")
  addInfo.Draw("same")
  p11_1.RedrawAxis()
  p11_1.Update()
  p11_1.GetFrame().Draw()
  CMS_lumi.CMS_lumi(p11_1, iPeriod, iPos)
  
  c1.cd(2)
  p11_2 = c1.GetPad(2)
  p11_2.SetPad(0.01,0.02,0.99,0.27)
  p11_2.SetBottomMargin(0.35)
  p11_2.SetRightMargin(0.05)
  p11_2.SetGridx()
  p11_2.SetGridy()
  vFrame2 = p11_2.DrawFrame(p11_1.GetUxmin(), -3.5, p11_1.GetUxmax(), 3.5)
  # vFrame2 = p11_2.DrawFrame(fFitXmin,-3.5,fFitXmax,3.5)
  vFrame2.SetTitle("")
  vFrame2.SetXTitle("M_{jj} [GeV]")
  vFrame2.GetXaxis().SetTitleSize(0.06)
  vFrame2.SetYTitle("#frac{Data-Fit}{#sigma}")
  vFrame2.GetYaxis().SetTitleSize(0.15)
  vFrame2.GetYaxis().SetTitleOffset(0.40)
  vFrame2.GetYaxis().SetLabelSize(0.09)
  vFrame2.GetXaxis().SetTitleSize(0.15)
  vFrame2.GetXaxis().SetTitleOffset(0.90)
  vFrame2.GetXaxis().SetLabelSize(0.12)

  hist_fit_residual_vsMass[0].GetXaxis().SetNdivisions(405)
  hist_fit_residual_vsMass[0].SetLineColor( kBlack)
  hist_fit_residual_vsMass[0].SetMarkerColor(kBlack)
  hist_fit_residual_vsMass[0].SetMarkerStyle(20)
  hist_fit_residual_vsMass[0].SetMarkerSize(1.)
  hist_fit_residual_vsMass[0].Draw("same")
  line = TLine(fFitXmin,0,fFitXmax,0)
  line.Draw("same")
  p11_2.RedrawAxis()
  line2=TLine()
  line2.DrawLine(p11_2.GetUxmin(), p11_2.GetUymax(), p11_2.GetUxmax(), p11_2.GetUymax())
  line2.DrawLine(p11_2.GetUxmax(), p11_2.GetUymin(), p11_2.GetUxmax(), p11_2.GetUymax())

  # time.sleep(10)
  cname = fOutputFile+"_fit.pdf"
  c1.SaveAs(cname)

  # del c1
# ---------------------------------------------------------------------------------------------------------------------------    
def FitComparisons(hMassNEW,g,M1Bkg,hist_fit_residual_vsMass,FunctionType,nPar,fFitXmin,fFitXmax,fLabel,fOutputFile,chi2,dof):
  W = 600
  H = 700
  H_ref = 700 
  W_ref = 600 
  T = 0.08*H_ref
  B = 0.12*H_ref
  L = 0.12*W_ref
  R = 0.04*W_ref

  c2 = TCanvas("c2","VV mass fit",W,H)
  c2.GetWindowHeight()
  c2.GetWindowWidth()
  c2.SetLogy()
  c2.Divide(1,2,0,0,0)

  #------------ pad 1  ----------------
  c2.cd(1)
  p11_1 = c2.GetPad(1)
  p11_1.SetPad(0.01,0.26,0.99,0.98)
  p11_1.SetLogy()
  p11_1.SetRightMargin(0.05)
  p11_1.SetTopMargin(0.05)
  p11_1.SetFillColor(0)
  p11_1.SetBorderMode(0)
  p11_1.SetFrameFillStyle(0)
  p11_1.SetFrameBorderMode(0)

  #Pave text
  #addInfo = TPaveText(0.1558691,0.30735043,0.3750171,0.4070085,"NDC")
  addInfo = TPaveText(0.2358691,0.04035043,0.5050171,0.1870085,"NDC")
  
  addInfo.AddText(fLabel)
  # addInfo.AddText("65 GeV < M_{P} < 105 GeV")
  # addInfo.AddText("|#eta| < 2.4, p_{T} > 200 GeV")
  addInfo.AddText("M_{jj} > 1 TeV, |#Delta#eta_{jj}| < 1.3")
  addInfo.SetFillColor(0)
  addInfo.SetLineColor(0)
  addInfo.SetFillStyle(0)
  addInfo.SetBorderSize(0)
  addInfo.SetTextFont(42)
  addInfo.SetTextSize(0.040)
  addInfo.SetTextAlign(12)
  
  vFrame = p11_1.DrawFrame(fFitXmin,0.000005,fFitXmax,hMassNEW.GetMaximum()*6.0)  
  vFrame.SetTitle("")
  vFrame.SetXTitle("M_{jj} [GeV]")
  vFrame.SetYTitle("#frac{dN}{dm_{jj}}")
  vFrame.GetXaxis().SetTitleSize(0.06)
  vFrame.GetXaxis().SetTitleOffset(0.95)
  vFrame.GetXaxis().SetLabelSize(0.05)
  vFrame.GetYaxis().SetTitleSize(0.06)
  #vFrame.GetYaxis().SetTitleOffset(1.0)
  vFrame.GetYaxis().SetLabelSize(0.05)

  g.SetMarkerSize(0.9)
  g.SetMarkerStyle(20)
  g.GetXaxis().SetNdivisions(405)
  g.Draw("pe0 same")
  
  colors = [kOrange,kRed,kBlue,kMagenta,kBlack,kGreen]
  styles = [2,1,4,8,2,6,8,7]
  i =0
  for f in M1Bkg:
    f.SetLineWidth(2)
    f.SetLineStyle(styles[i])
    f.SetLineColor(colors[i])
    f.Draw("same")
    i+=1

  legend = TLegend(0.337351,0.7177579,0.5890289,0.9182374)
  legend.SetTextSize(0.038)
  legend.SetLineColor(0)
  legend.SetShadowColor(0)
  legend.SetLineStyle(1)
  legend.SetLineWidth(1)
  legend.SetFillColor(0)
  legend.SetFillStyle(0)
  legend.SetMargin(0.35)
  legend.AddEntry(g, "CMS data","lpe")
  legend.AddEntry(M1Bkg[0], "Default fit, 2 par.   (#chi^{2}/ndof = %.2f/%i)"%(chi2[0],dof[0]),"l")
  legend.AddEntry(M1Bkg[1], "Default fit, 3 par.   (#chi^{2}/ndof = %.2f/%i)"%(chi2[1],dof[1]),"l")
  legend.AddEntry(M1Bkg[2], "Default fit, 4 par.   (#chi^{2}/ndof = %.2f/%i)"%(chi2[2],dof[2]),"l")
  legend.AddEntry(M1Bkg[3], "Default fit, 5 par.   (#chi^{2}/ndof = %.2f/%i)"%(chi2[3],dof[3]),"l")
  legend.AddEntry(M1Bkg[4], "Alternate fit, 4 par. (#chi^{2}/ndof = %.2f/%i)"%(chi2[4],dof[4]),"l")
  legend.Draw("same")
  addInfo.Draw("same")
  p11_1.RedrawAxis()
  p11_1.Update()
  p11_1.GetFrame().Draw()
  CMS_lumi.CMS_lumi(p11_1, iPeriod, iPos)
  
  c2.cd(2)
  p11_2 = c2.GetPad(2)
  p11_2.SetPad(0.01,0.02,0.99,0.27)
  p11_2.SetBottomMargin(0.35)
  p11_2.SetRightMargin(0.05)
  p11_2.SetGridx()
  p11_2.SetGridy()
  vFrame2 = p11_2.DrawFrame(p11_1.GetUxmin(), -3.5, p11_1.GetUxmax(), 3.5)
  vFrame2.SetTitle("")
  vFrame2.SetXTitle("M_{jj} [GeV]")
  vFrame2.GetXaxis().SetTitleSize(0.06)
  vFrame2.SetYTitle("#frac{Data-Fit}{#sigma}")
  vFrame2.GetYaxis().SetTitleSize(0.15)
  vFrame2.GetYaxis().SetTitleOffset(0.40)
  vFrame2.GetYaxis().SetLabelSize(0.09)
  vFrame2.GetXaxis().SetTitleSize(0.15)
  vFrame2.GetXaxis().SetTitleOffset(0.90)
  vFrame2.GetXaxis().SetLabelSize(0.12)
  vFrame2.GetXaxis().SetNdivisions(405)
  
  styles = [20,22,32,25,32]
  i =0
  for h in hist_fit_residual_vsMass:   
    h.SetLineColor(colors[i])
    h.SetMarkerColor(colors[i])
    h.SetMarkerStyle(styles[i])
    h.SetMarkerSize(1.)
    h.Draw("same")
    i+=1
  line = TLine(fFitXmin,0,fFitXmax,0)
  line.Draw("same")
  p11_2.RedrawAxis()
  line2=TLine()
  line2.DrawLine(p11_2.GetUxmin(), p11_2.GetUymax(), p11_2.GetUxmax(), p11_2.GetUymax())
  line2.DrawLine(p11_2.GetUxmax(), p11_2.GetUymin(), p11_2.GetUxmax(), p11_2.GetUymax())
  
  cname = fOutputFile+"_fitComp.pdf"
  c2.SaveAs(cname)
  time.sleep(100)

  # del c2
  
# ---------------------------------------------------------------------------------------------------------------------------
if __name__ == '__main__':
  

  massBins =[1, 3, 6, 10, 16, 23, 31, 40, 50, 61, 74, 88, 103, 119, 137, 156, 176, 197, 220, 244, 270, 296, 325, 354, 386, 419, 453, 489, 526, 565, 606, 649, 693, 740, 788, 838, 890, 944, 1000, 1058,
             1118, 1181, 1246, 1313, 1383, 1455, 1530, 1607, 1687, 1770, 1856, 1945, 2037, 2132, 2231, 2332, 2438, 2546, 2659, 2775, 2895, 3019, 3147, 3279, 3416, 3558, 3704, 3854, 4010, 4171, 4337, 
             4509, 4686, 4869, 5058, 5253, 5455, 5663, 5877, 6099, 6328, 6564, 6808]

  channels = ["VV","WW","WZ","ZZ"]
  channels = ["ZZ"]
  for ch in channels:
    if ch.find("q") != -1: 
      fitmax = 5058
    else: 
      fitmax = 5058
    
    # performFit("input/QCD.root",
    #          "DijetMassHighPuri%s"%ch, len(massBins)-1, massBins, 1000, fitmax, "%s category, HP"%ch,
    #          "%s_HP_QCD"%ch)
    performFit("input/QCD.root",
               "DijetMassLowPuri%s"%ch, len(massBins)-1, massBins, 1000, fitmax, "%s category, LP"%ch,
               "%s_LP_QCD"%ch)
    # performFit("input/QCD.root",
    #            "DijetMassNoPuri%s"%ch, len(massBins)-1, massBins, 1001, fitmax, "%s category, NP"%ch,
    #            "plots/%s_NP_QCD"%ch)  
    # performFit("input/DATA_SB.root",
    #          "DijetMassHighPuri%s"%ch, len(massBins)-1, massBins, 1000, fitmax, "%s category, HP"%ch,
    #          "plots/%s_HP_1263invpb.pdf"%ch)
#     performFit("input/DATA.root",
#                "DijetMassLowPuri%s"%ch, len(massBins)-1, massBins, 1000, fitmax, "%s category, LP"%ch,
#                "plots/%s_LP_1263invpb.pdf"%ch)
    # performFit("input/DATA_SB.root",
    #            "DijetMassNoPuri%s"%ch, len(massBins)-1, massBins, 1000, fitmax, "%s category, NP"%ch,
    #            "plots/%s_NP_1263invpb_SB.pdf"%ch)
