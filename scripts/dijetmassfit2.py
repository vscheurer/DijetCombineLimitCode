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

tdrstyle.setTDRStyle()
gStyle.SetOptFit(0) 
CMS_lumi.lumi_13TeV = "225.6 pb^{-1}"
CMS_lumi.writeExtraText = 1
CMS_lumi.extraText = "Preliminary"
CMS_lumi.lumi_sqrtS = "13 TeV" # used with iPeriod = 0, e.g. for simulation-only plots (default is an empty string)
iPos = 11
if( iPos==0 ): CMS_lumi.relPosX = 0.12
iPeriod = 4

# ---------------------------------------------------------------------------------------------------------------------------
alpha = 1 - 0.6827
sqrtS = 13000.
lumi = 225.

def bkgFit3par(m, p):
  x=m[0]/sqrtS
  return p[0]*pow(1.-x,p[1])/pow(x,p[2])
  return bkgFit4par
  
def bkgFit4par(m, p):
  x=m[0]/sqrtS
  return p[0]*pow(1.-x,p[1])/pow(x,p[2]+p[3]*TMath.Log(x))
  return bkgFit4par

def bkgFit6par(m, p):
  x=m[0]/sqrtS
  return p[0]*pow(1.-x,p[1])/pow(x,p[2]+p[3]*TMath.Log(x)+p[4]*pow(TMath.Log(x),2)+p[5]*pow(TMath.Log(x),3))
  return bkgFit6par

def AltbkgFit4par(m, p):
  x=m[0]/sqrtS
  return p[0]*pow(1.-x +p[3]*x*x,p[1])/pow(x,p[2])
  return AltbkgFit4par
  
def performFit(fInputFile, fPlot, fNbins, fBins,fFitXmin, fFitXmax,fLabel,  fOutputFile,  use6ParFit, fP0=1e-04, fP1=1e+01,fP2=4e+00,fP3=-0.1e-01, fP4=0, fP5=0):
  
  # TVirtualFitter.SetMaxIterations(30000)
  bins = sorted(fBins)
  xbins = array('d',bins)
  
# --------------------------------------- Rebinning-->hMass_rebinned ---------------------------------------
  file =TFile(fInputFile,"READ")
  hMass = file.Get(fPlot)
  hMass.SetBinErrorOption(TH1.kPoisson)
  hMass.Scale(1/lumi)
  hMass.Rebin(2)

  FunctionTypes = [0] 
  for FunctionType in FunctionTypes: 
    fitresult = doFit(FunctionType,hMass,fFitXmin,fFitXmax,fNbins,xbins) 
    M1Bkg = fitresult[3]
    hist_fit_residual_vsMass = fitresult[4]
    DrawFit(hMass,M1Bkg,hist_fit_residual_vsMass,FunctionType,fFitXmin,fFitXmax)
    
    
def doFit(FunctionType,hMass,fFitXmin,fFitXmax,fNbins,xbins):
  if( FunctionType==0 ):    
    nPar=4
    BKGfit = TF1("BKGfit","( [0]*TMath::Power(1-x/13000,[1]) ) / ( TMath::Power(x/13000,[2]+[3]*log(x/13000)) )",fFitXmin,fFitXmax)
    # BKGfit.SetParLimits(0,0.,10)
    
    stopProgram=1;
    for loop in range (0,10):
      r = hMass.Fit("BKGfit","ELSR","",fFitXmin,fFitXmax)          
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
    NumberOfObservations_VarBin_5entries = 0
    chi2_VarBin = 0.
    chi2_VarBin_notNorm = 0.
    chi2_VarBin_5entries = 0.
    chi2_VarBin_zeroes = 0. 
    
    print hMass.GetBinWidth(20)
    print hMass.GetNbinsX()

    hist_fit_residual_vsMass =  TH1D("hist_fit_residual_vsMass","hist_fit_residual_vsMass",fNbins,xbins)
    print hist_fit_residual_vsMass.GetBinWidth(20)
    print hist_fit_residual_vsMass.GetNbinsX()
    
    for bin in range (1,hMass.GetNbinsX()):
      if( hMass.GetXaxis().GetBinLowEdge(bin)>=fFitXmin and hMass.GetXaxis().GetBinUpEdge(bin)<=fFitXmax ):
             NumberOfVarBins += 1
             #print "bin content = " + str(hMass.GetBinContent(bin)) + "   graph y = " + str(vy[bin-1]) + "  error y low = " + str(g.GetErrorYlow(bin-1))
             data = hMass.GetBinContent(bin)
             # err_data_low = hMass.GetErrorYlow(bin-1)
             # err_data_high= hMass.GetErrorYhigh(bin-1)
             err_data_low =hMass.GetBinErrorLow(bin)
             err_data_high = hMass.GetBinErrorUp(bin)
             fit = BKGfit.Integral(hMass.GetXaxis().GetBinLowEdge(bin) , hMass.GetXaxis().GetBinUpEdge(bin) )
             fit = fit / ( hMass.GetBinWidth(bin) )
             if(fit > data):
               err_tot = err_data_high
             else:
               err_tot = err_data_low
             fit_residual = (data - fit) / err_tot
             err_fit_residual = 1
             ##skip bin with zero entries
             chi2_VarBin_zeroes += pow( (data - fit) , 2 ) / pow( err_tot , 2 )
             if (hMass.GetBinContent(bin)>0):
               NumberOfObservations_VarBin+=1
               chi2_VarBin += pow( (data - fit) , 2 ) / pow( err_tot , 2 )	 
               chi2_VarBin_notNorm += pow( (data - fit) , 2 ) 	 
  
             ##skip bin with less than 5 entries
             if (hMass.GetBinContent(bin)*lumi*hMass.GetBinWidth(bin)>=5):
               	NumberOfObservations_VarBin_5entries+=1
                chi2_VarBin_5entries += pow( (data - fit) , 2 ) / pow( err_tot , 2 )	 
  
             hist_fit_residual_vsMass.SetBinContent(bin,fit_residual)
             hist_fit_residual_vsMass.SetBinError(bin,err_fit_residual)
    
    ndf_VarBin_5entries = NumberOfObservations_VarBin_5entries - nPar# -1
    ndf_VarBin = NumberOfObservations_VarBin - nPar# -1
    ndf_VarBin_withzeroes = NumberOfVarBins - nPar# -1
    print "============================" 
    print "NumberOfObservations_VarBin: %d" %  NumberOfObservations_VarBin
    print "ndf_VarBin: %d" % ndf_VarBin 
    print "ndf_VarBin with zeroes: %d" % ndf_VarBin_withzeroes 
    print "ndf_VarBin with 5entries: %d" % ndf_VarBin_5entries
    print "chi2_VarBin with zeroes: %f" % chi2_VarBin_zeroes
    print "chi2_VarBin: %f" % chi2_VarBin
    print "chi2_VarBin_5entries: %f" % chi2_VarBin_5entries
    print "chi2_VarBin_notNorm: %f" % chi2_VarBin_notNorm
    print "============================"   
    
    return [chi2_VarBin_notNorm,ndf_VarBin,chi2_VarBin,BKGfit,hist_fit_residual_vsMass]   
    
 
# --------------------------------------- Do fit!-->fit -------------------  
  # fit = TF1("fit",bkgFit4par,fFitXmin,fFitXmax,nPar)           # 4 Param. Fit
  # if(use6ParFit):
  #   nPar = 6
  #   fit = TF1("fit",bkgFit6par,fFitXmin,fFitXmax,nPar) # 6 Param. Fit
  # # fit.FixParameter(0,fP0)
  # # fit.FixParameter(1,fP1)
  # # fit.FixParameter(2,fP2)
  # # fit.FixParameter(3,fP3)
  # if(use6ParFit):
  #   fit.FixParameter(4,fP4)
  #   fit.FixParameter(5,fP5)
  # fit.FixParameter(3,fP3)
  # print "*********************************************************"
  # s = TFitResultPtr(hMass.Fit("fit","SRLI"))
  # # status_default = gMinuit.fCstatu.Data()
  # # // Results of the fit
  # print "*********************************************************"
  # chi_fit = fit.GetChisquare()
  # ndf_fit = fit.GetNDF()
  # print "Chi2/ndf: %f/%i = %f"%(chi_fit,ndf_fit,chi_fit/ndf_fit)
  # # print status_default
  # print "*********************************************************"
  #
  # # // Print fit results
  # s.Print("V")
  # fit.SetLineWidth(2)
  # fit.SetLineColor(kBlue)
  
#--------------Compute residuals-------------  
  
   
# Draw plots
def DrawFit(hMass,M1Bkg,hist_fit_residual_vsMass,FunctionType,fFitXmin,fFitXmax):
  W = 600
  H = 700
  H_ref = 700 
  W_ref = 600 
  T = 0.08*H_ref
  B = 0.12*H_ref
  L = 0.12*W_ref
  R = 0.04*W_ref

  c = TCanvas("c","VV mass fit",W,H)
  c.GetWindowHeight()
  c.GetWindowWidth()
  c.SetLogy()
  c.Divide(1,2,0,0,0)

  #------------ pad 1  ----------------
  c.cd(1)
  p11_1 = c.GetPad(1)
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
  
  # addInfo.AddText("AK8CHSPF jets")
  addInfo.AddText("65 GeV Pruned M < 105 GeV")
  # addInfo.AddText("|#eta| < 2.4, p_{T} > 200 GeV")
  addInfo.AddText("M_{jj} > 1 TeV, |#Delta#eta_{jj}| < 1.3")
  addInfo.SetFillColor(0)
  addInfo.SetLineColor(0)
  addInfo.SetFillStyle(0)
  addInfo.SetBorderSize(0)
  addInfo.SetTextFont(42)
  addInfo.SetTextSize(0.040)
  addInfo.SetTextAlign(12)
  
  vFrame = p11_1.DrawFrame(fFitXmin,0.000005,fFitXmax,hMass.GetMaximum()*5.0)  
  vFrame.SetTitle("")
  vFrame.SetXTitle("M_{jj} [GeV]")
  vFrame.SetYTitle("#frac{d#sigma}{dm_{jj}}")
  vFrame.GetXaxis().SetTitleSize(0.06)
  vFrame.GetXaxis().SetTitleOffset(0.95)
  vFrame.GetXaxis().SetLabelSize(0.05)
  vFrame.GetYaxis().SetTitleSize(0.06)
  #vFrame.GetYaxis().SetTitleOffset(1.0)
  vFrame.GetYaxis().SetLabelSize(0.05)

  hMass.GetXaxis().SetNdivisions(405)
  hMass.SetMarkerSize(0.9)
  hMass.SetMarkerStyle(20)
  # g.GetXaxis().SetNdivisions(405)
  M1Bkg.SetLineWidth(2)
  M1Bkg.SetLineStyle(1)
  M1Bkg.SetLineColor(2)
  M1Bkg.Draw("same")
  hMass.Draw("ME0same")

  legend = TLegend(0.6564991,0.72,0.8703575,0.80)
  legend.SetTextSize(0.038)
  legend.SetLineColor(0)
  legend.SetShadowColor(0)
  legend.SetLineStyle(1)
  legend.SetLineWidth(1)
  legend.SetFillColor(0)
  legend.SetFillStyle(0)
  legend.SetMargin(0.35)
  legend.AddEntry(hMass, "Data","lpe")
  legend.AddEntry(M1Bkg, "Fit","l")
  legend.Draw("same")
  addInfo.Draw("same")
  p11_1.RedrawAxis()
  p11_1.Update()
  p11_1.GetFrame().Draw()
  CMS_lumi.CMS_lumi(p11_1, iPeriod, iPos)
  
  c.cd(2)
  p11_2 = c.GetPad(2)
  p11_2.SetPad(0.01,0.02,0.99,0.27)
  p11_2.SetBottomMargin(0.35)
  p11_2.SetRightMargin(0.05)
  p11_2.SetGridx()
  p11_2.SetGridy()
  vFrame2 = p11_2.DrawFrame(p11_1.GetUxmin(), -3.5, p11_1.GetUxmax(), 3.5)
  # vFrame2 = p11_2.DrawFrame(fFitXmin,-3.5,fFitXmax,3.5)
  vFrame2.SetTitle("")
  vFrame2.SetXTitle("Dijet Mass [GeV]")
  vFrame2.GetXaxis().SetTitleSize(0.06)
  vFrame2.SetYTitle("#frac{Data-Fit}{#sigma}")
  vFrame2.GetYaxis().SetTitleSize(0.15)
  vFrame2.GetYaxis().SetTitleOffset(0.40)
  vFrame2.GetYaxis().SetLabelSize(0.09)
  vFrame2.GetXaxis().SetTitleSize(0.15)
  vFrame2.GetXaxis().SetTitleOffset(0.90)
  vFrame2.GetXaxis().SetLabelSize(0.12)
  
  hist_fit_residual_vsMass.GetXaxis().SetNdivisions(405)
  # hist_fit_residual_vsMass.GetXaxis().SetRangeUser(fFitXmin,fFitXmax)
  # hist_fit_residual_vsMass.GetYaxis().SetRangeUser(-3.5,3.5)
  # hist_fit_residual_vsMass.SetLineWidth(0)
  # hist_fit_residual_vsMass.SetFillColor(2)
  # hist_fit_residual_vsMass.SetLineColor(1)
  # hist_fit_residual_vsMass.SetLineColor( kBlack)   
  hist_fit_residual_vsMass.SetLineColor( kBlack)
  hist_fit_residual_vsMass.SetMarkerColor( kBlack)
  hist_fit_residual_vsMass.SetMarkerStyle(20)
  hist_fit_residual_vsMass.SetMarkerSize(1.)
  hist_fit_residual_vsMass.Draw("same")

  line = TLine(fFitXmin,0,fFitXmax,0)
  line.Draw("same")
  p11_2.RedrawAxis()
  line2=TLine()
  line2.DrawLine(p11_2.GetUxmin(), p11_2.GetUymax(), p11_2.GetUxmax(), p11_2.GetUymax())
  line2.DrawLine(p11_2.GetUxmax(), p11_2.GetUymin(), p11_2.GetUxmax(), p11_2.GetUymax())

  time.sleep(100)
  c.SaveAs(fOutputFile)

  del c
  del file
    



if __name__ == '__main__':
  
  number_of_variableWidth_bins = 103

  massBins =[1, 3, 6, 10, 16, 23, 31, 40, 50, 61, 74, 88, 103, 119, 137, 156, 176, 197, 220, 244, 270, 296, 325, 354, 386, 419, 453, 489, 526, 
           565, 606, 649, 693, 740, 788, 838, 890, 944, 1000, 1058, 1118, 1181, 1246, 1313, 1383, 1455, 1530, 1607, 1687, 1770, 1856, 1945, 2037, 
           2132, 2231, 2332, 2438, 2546, 2659, 2775, 2895, 3019, 3147, 3279, 3416, 3558, 3704, 3854, 4010, 4171, 4337, 4509, 4686, 4869, 5058, 5253, 
           5455, 5663, 5877, 6099, 6328, 6564, 6808, 7060, 7320, 7589, 7866, 8152, 8447, 8752, 9067, 9391, 9726, 10072, 10430,10798, 11179, 11571, 11977, 12395, 12827, 13272, 13732, 14000]
           
  
  
  
  bins = [ 1000, 1181, 1246, 1313, 1383, 1455, 1530, 1607, 1687,1770, 1856, 1945, 2037, 2132, 2231, 2332, 2438, 2546,
           2659, 2775, 2895, 3019, 3147, 3279, 3416, 3558, 3704,3854, 4010, 4171, 4337, 4509, 4686, 4869, 5000]
  
  performFit("DATA.root",
             "DijetMassHighPuriVV", len(massBins)-1, massBins, 1000, 4000, "M_{jj}>1000 GeV",
             "VV-3ParFit_13TeV_225pb.pdf", 0, 8.62449e-04, 1.02685e+01, 5.00842e+00,  0.00000e+00)