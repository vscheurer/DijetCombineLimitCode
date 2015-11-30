import os,commands, os.path
import sys
sys.argv.append( '-b-' )
from ROOT import *
import math
import time
from array import *
import CMS_lumi, tdrstyle

gStyle.SetOptTitle(0)
# gStyle.SetOptStat("rme")
gStyle.SetOptStat(0)

def doPull(hSB,hB,gSB,fNbins,xbins,fFitXmin,fFitXmax):
  myPull = TH1F("myPull","myPull",fNbins,xbins)
  for b in xrange(1,fNbins):
    if( hSB.GetXaxis().GetBinLowEdge(b)>=fFitXmin and hSB.GetXaxis().GetBinUpEdge(b)<=fFitXmax ):
      nbkg = hB.GetBinContent(b)
      ndata = hSB.GetBinContent(b)
      if ndata != 0:
        err_data_low = gSB.GetErrorYlow(b-1) 
        err_data_high= gSB.GetErrorYhigh(b-1)
        if(nbkg > ndata):
          err = err_data_high
        else:
          err = err_data_low  
        # err = TMath.Sqrt( h.GetBinError(b)*hdata.GetBinError(b) + histsum.GetBinError(b)*histsum.GetBinError(b) )
        p = (ndata-nbkg)/err
        myPull.SetBinContent(b,p)
        myPull.SetBinError(b,1) 
  myPull.SetLineColor(kBlack)      
  myPull.SetLineColor(kBlack)
  myPull.SetMarkerColor(kBlack)
  myPull.SetMarkerStyle(20)
  myPull.SetMarkerSize(1.)
  myPull.SetMinimum(-6.)
  myPull.SetMaximum(6.)
  myPull.GetYaxis().SetTitle("#frac{Data-MC}{#sigma}")
  myPull.GetYaxis().SetNdivisions(4)
  myPull.GetYaxis().SetLabelSize(0.15)
  myPull.GetXaxis().SetLabelSize(0.15)
  myPull.GetYaxis().SetTitleSize(0.2)
  myPull.GetYaxis().SetTitleOffset(0.2)
  myPull.GetYaxis().CenterTitle()
  return myPull
   
def write(fname, histolist):
    "Write the new histogram to disk"
    base = fname
    outfname = "/shome/thaarres/EXOVVAnalysisRunII/LimitCode/CMSSW_7_1_5/src/DijetCombineLimitCode/input/Pseudodata.root"
    print "Saving file %s " %outfname
    fout = TFile(outfname,"RECREATE")
    for h in histolist:
      h.Write()
    fout.Close()


channels = ["VVHP","WWHP","WZHP","ZZHP"]
# channels = ["WZHP"]
ntoys = 100
scaleS=1.0

infile  = "/shome/thaarres/EXOVVAnalysisRunII/LimitCode/CMSSW_7_1_5/src/DijetCombineLimitCode/input/QCD_new.root"
infileS = "/shome/thaarres/EXOVVAnalysisRunII/LimitCode/CMSSW_7_1_5/src/DijetCombineLimitCode/input/WprimeToWZ_13TeV_1400GeV.root"

massBins =[1, 3, 6, 10, 16, 23, 31, 40, 50, 61, 74, 88, 103, 119, 137, 156, 176, 197, 220, 244, 270, 296, 325, 354, 386, 419, 453, 489, 526, 565, 606, 649, 693, 740, 788, 838, 890, 944, 1000, 1058,
             1118, 1181, 1246, 1313, 1383, 1455, 1530, 1607, 1687, 1770, 1856, 1945, 2037, 2132, 2231, 2332, 2438, 2546, 2659, 2775, 2895, 3019, 3147, 3279, 3416, 3558, 3704, 3854, 4010, 4171, 4337, 
             4509, 4686, 4869, 5058, 5253, 5455, 5663, 5877, 6099, 6328, 6564, 6808]
                       
xbins = array('d',massBins)
fNbins = len(massBins)-1

xsec_Wprime2TeV = 1
lumi = 2460.0
fitmin = 1000
fitmax = 2332    

print "Starting bias study for channels %s" %channels


histnames = [ 'DijetMassHighPuriVV',
              'DijetMassHighPuriWW',
              'DijetMassHighPuriWZ',
              'DijetMassHighPuriZZ',
              'DijetMassLowPuriVV',
              'DijetMassLowPuriWW',
              'DijetMassLowPuriWZ',
              'DijetMassLowPuriZZ',
              'DijetMassHighPuriqV',
              'DijetMassHighPuriqW',
              'DijetMassHighPuriqZ',
              'DijetMassLowPuriqV',
              'DijetMassLowPuriqW',
              'DijetMassLowPuriqZ',
              'DijetMassNoPuriVV',
              'DijetMassNoPuriWW',
              'DijetMassNoPuriWZ',
              'DijetMassNoPuriZZ',
              'DijetMassNoPuriqV',
              'DijetMassNoPuriqW',
              'DijetMassNoPuriqZ'
            ]
            
bkgyields = []
fits = []
sighists = []
mjjcanv = []

filetmpS = TFile.Open(infileS,"READ")
filetmpB = TFile.Open(infile,"READ")

for j in range(0,len(histnames)):
  histname = histnames[j]
  histB = filetmpB.Get(histname)
  histB.SetName("%s_B" %histB.GetName())
  bkgyield = int(histB.Integral())
  bkgyields.append(bkgyield)
  
  htmp = histB.Rebin(fNbins,"htmp",xbins)
  BKGfit = TF1("BKGfit","( [0]*TMath::Power(1-x/13000,[1]) ) / ( TMath::Power(x/13000,[2]) )",fitmin,fitmax)
  htmp.Fit("BKGfit","ISR","",fitmin,2332)
  BKGfit.SetName(histname)
  fits.append(BKGfit)
  
  hs = TH1F('hs','hs',7000,0,7000)
  histS = filetmpS.Get(histname)
  histS.Scale(lumi*scaleS)
  for j in range (0, int(histS.Integral())):
    hs.Fill(histS.GetRandom())  
  hs.SetName("%s_S" %histname)
  sighists.append(hs)
  print histS.GetName()
  print "BEFORE: Nr signal events == %.2f"%histS.Integral()
  print "AFTER:  Nr signal events == %.2f"%hs.Integral()
  
# filetmpB.Close()
# filetmpS.Close()
# del histB
# del htmp
# del histS

hlist_Bias=[]
hlist_Pull=[]
hlist_RSS=[]
hlist_SigmaObs=[]
hlist_SigmaExp=[]

for ch in channels:
  bias = TH1F("bias%s"%ch,"bias%s"%ch,100,-0.1,5.0)
  pull = TH1F("pull%s"%ch,"pull%s"%ch,100,-3.,3.)
  residuals = TH1F("residuals%s"%ch,"residuals%s"%ch,100,-4.0,4.0)
  sigmaObs = TH1F("sigmaObs%s"%ch,"sigmaObs%s"%ch,100,-1.,15.)
  sigmaExp = TH1F("sigmaExp%s"%ch,"sigmaExp%s"%ch,100,-1.,15.)
  hlist_Bias.append(bias)
  hlist_Pull.append(pull)
  hlist_RSS.append(residuals)
  hlist_SigmaObs.append(sigmaObs)
  hlist_SigmaExp.append(sigmaExp)

gRandom = TRandom3(0)
for ii in range (0,ntoys):
  print ""
  print "Computing extracted signal strength and significance for toy %i"%ii
  print ""
  histolist=[]
  for i in range(0,len(histnames)):
    histname = histnames[i]
    print histname
    hB = TH1F('%s'%histname,'%s'%histname,7000,0,7000)
    for j in range(0,bkgyields[i]):
      hB.Fill(fits[i].GetRandom())
    
    hSB = hB.Clone()
    hSB.SetName(histname)
    hsig = sighists[i].Clone()
    hsig.SetName("sighist")
    hSB.Add(hsig)
    # histolist[i].Fill(1400,103)
    for e in range(1,hSB.GetNbinsX()):
      error = math.sqrt(hSB.GetBinContent(e))
      hSB.SetBinError(e,error)
            
    histolist.append(hSB)
    
    if ii==0:
      cSB = TCanvas("%s"%histname,"%s"%histname,600,700)
      cSB.GetWindowHeight()
      cSB.GetWindowWidth()
      cSB.SetLogy()
      cSB.Divide(1,2,0,0,0)
      cSB.cd(1)
      pad1 = cSB.GetPad(1)
      pad1.SetPad(0.01,0.26,0.99,0.98)
      pad1.SetLogy()
      pad1.SetRightMargin(0.05)
      pad1.SetTopMargin(0.05)
      pad1.SetFillColor(0)
      pad1.SetBorderMode(0)
      pad1.SetFrameFillStyle(0)
      pad1.SetFrameBorderMode(0)
    
      addInfo = TPaveText(0.2358691,0.04035043,0.5050171,0.1870085,"NDC")
      addInfo.AddText("%s"%histname)
      addInfo.SetFillColor(0)
      addInfo.SetLineColor(0)
      addInfo.SetFillStyle(0)
      addInfo.SetBorderSize(0)
      addInfo.SetTextFont(42)
      addInfo.SetTextSize(0.040)
      addInfo.SetTextAlign(12)
      
      hsig = hsig.Rebin(fNbins,"hB",xbins)
      hB = hB.Rebin(fNbins,"hB",xbins)
      htmp = hSB.Clone()
      htmp = htmp.Rebin(fNbins,"hBackground",xbins)
      hsig.SetLineColor(kRed)
      hB.SetFillStyle(3002)
      hB.SetFillColor(kBlack)
      hB.SetLineColor(kBlack)
      
      frame1 = pad1.DrawFrame(fitmin,0.01,fitmax,htmp.GetMaximum()*8)
      frame1.SetTitle("")
      frame1.SetXTitle("Dijet invariant mass [GeV]")
      frame1.SetYTitle("Nr. toys")
      frame1.GetYaxis().SetTitleOffset(0.85)
      frame1.GetXaxis().SetTitleSize(0.06)
      frame1.GetXaxis().SetTitleOffset(0.95)
      frame1.GetXaxis().SetLabelSize(0.05)
      frame1.GetYaxis().SetTitleSize(0.06)
      frame1.GetYaxis().SetLabelSize(0.05)
      frame1.GetXaxis().SetNdivisions(405)
      hsig.Draw("histSAME")
      hB.Draw("HISTsame")
      
      x=[]
      y=[]
      exl=[]
      exh=[]
      eyl=[]
      eyh=[]
      alpha = 1 - 0.6827
      for k in range(0,htmp.GetNbinsX()):
        n    = htmp.GetBinContent(k+1)
        dm   = htmp.GetBinWidth(k+1)
        mass = htmp.GetBinCenter(k+1)
        xl   = htmp.GetBinLowEdge(k+1)
        xh   = xl+dm
        x.append( (xl+xh)/2.)
        exl.append( dm/2.)
        exh.append( dm/2.)
        y.append( n)
        if(n==0): l = 0
        else: l = Math.gamma_quantile(alpha/2,n,1.) #as recommended in https://twiki.cern.ch/twiki/bin/view/CMS/PoissonErrorBars
        h = Math.gamma_quantile_c(alpha/2,n+1,1)
        eDATA_L = (n-l)
        eDATA_H = (h-n)
        eyl.append( (n-l) )
        eyh.append( (h-n) )
        
      vx = array("f",x)
      vy = array("f",y)
      vexl = array("f",exl)
      vexh = array("f",exh)
      veyl = array("f",eyl)
      veyh = array("f",eyh)
      gSB = TGraphAsymmErrors(fNbins,vx,vy,vexl,vexh,veyl,veyh)
      gSB.SetName("gSB")
      gSB.SetMarkerSize(0.9)
      gSB.SetMarkerStyle(20)
      gSB.Draw("pe0 same")
      addInfo.Draw("same")

      legend = TLegend(0.50258006,0.7136243,0.77074784,0.9141038)
      legend.SetTextSize(0.038)
      legend.SetLineColor(0)
      legend.SetShadowColor(0)
      legend.SetLineStyle(1)
      legend.SetLineWidth(1)
      legend.SetFillColor(0)
      legend.SetFillStyle(0)
      legend.SetMargin(0.35)
      legend.AddEntry(hB, "Pseudodata (B)","f")
      legend.AddEntry(hsig, "W' (1.4 TeV) (%i events)"%hsig.Integral(),"l")
      legend.AddEntry(gSB, "Pseudodata (S+B)","p")
      legend.Draw("same")
      cSB.Update()

      cSB.cd(2)
      pad2 = cSB.GetPad(2)
      pad2.SetPad(0.01,0.02,0.99,0.27)
      pad2.SetBottomMargin(0.35)
      pad2.SetRightMargin(0.05)
      pad2.SetGridx()
      pad2.SetGridy()
      frame2 = pad2.DrawFrame(pad1.GetUxmin(), -6.8, pad1.GetUxmax(), 6.8)
      frame2.SetTitle("")
      frame2.SetXTitle("Dijet invariant mass [GeV]")
      frame2.GetXaxis().SetTitleSize(0.06)
      frame2.SetYTitle("#frac{(S+B)-B}{#sigma}")
      frame2.GetYaxis().SetTitleSize(0.15)
      frame2.GetYaxis().SetTitleOffset(0.25)
      frame2.GetYaxis().SetLabelSize(0.09)
      frame2.GetXaxis().SetTitleSize(0.15)
      frame2.GetXaxis().SetTitleOffset(0.90)
      frame2.GetXaxis().SetLabelSize(0.12)
      frame2.GetXaxis().SetNdivisions(405)
      frame2.GetYaxis().SetNdivisions(405)
      h1 = doPull (htmp,hB,gSB,fNbins,xbins,fitmin,fitmax)
      h1.Draw("same")
      cSB.Update()
      line = TLine(fitmin,0,fitmax,0)
      line.Draw("same")
      pad2.RedrawAxis()
      line2=TLine()
      line2.DrawLine(pad2.GetUxmin(), pad2.GetUymax(), pad2.GetUxmax(), pad2.GetUymax())
      cSB.Update()
      mjjcanv.append(cSB)
      cname = histname +"Wprime_%s"%scaleS+"pb_Mjj.pdf"
      cSB.SaveAs(cname)
      del htmp
      del hB
      del hsig
      del gSB

  for h in histolist:
    print "Saving histogram %s" %h.GetName()
    write("Pseudodata",histolist)

  cmd = "root -l -q MiniTreeProducerVV13TeV.C"
  print cmd
  os.system(cmd)
  cmd = "python ProduceWorkspaces13TeV.py"
  print cmd
  os.system(cmd)

  i =-1
  for ch in channels:
    i += 1
    cmd = "combine datacards/CMS_jj_WZ_1400_13TeV_CMS_jj_%s.txt  -M MaxLikelihoodFit -v2 -m 1400 --rMax 100 --rMin -100 > tmp.txt"%ch
    print cmd
    os.system(cmd)
    cmd = "combine -M ProfileLikelihood -n %sObsSignif -m 1400  --signif --pvalue -d datacards/CMS_jj_WZ_1400_13TeV_CMS_jj_%s.txt > tmp2.txt"%(ch,ch)
    print cmd
    os.system(cmd)
    cmd = "combine -M ProfileLikelihood -n %sExpSignif -m 1400  -s 1 --signif --pvalue --expectSignal=1 -t -1 --toysFreq -d datacards/CMS_jj_WZ_1400_13TeV_CMS_jj_%s.txt > tmp3.txt" %(ch,ch)
    print cmd
    os.system(cmd)
    valid_input = True
    with open("tmp.txt") as f:
      for line in f.readlines():
        # if "Minimized function has error status." in line: valid_input = False
        if "Best fit" in line:
          rate = line
    with open("tmp2.txt") as f:
      for line in f.readlines():
        # if "Minimized function has error status." in line: valid_input = False
        if "Significance" in line:
          obssig = line
    with open("tmp3.txt") as f:
      for line in f.readlines():
        # if "Minimized function has error status." in line: valid_input = False
        if "Significance" in line:
          expsig = line
    if valid_input :
      ObsSig = float ((obssig.split("= ")[1]).replace(")",""))
      ExpSig = float ((expsig.split("= ")[1]).replace(")",""))
      hlist_SigmaObs[i].Fill(ObsSig)
      hlist_SigmaExp[i].Fill(ExpSig)
      r = float(rate.split(" ")[3])
      hlist_Bias[i].Fill(r)
      err = rate.split(" ")[5]
      eH = err.split("/")[1]
      if (eH.find("+-")!=-1): 
        eH = float(eH.split("-")[1])
      else:
        eH = float(eH)  
      eL = err.split("/")[0]
      if (eL.find("+-")!=-1): 
        eL = float(eL.split("-")[1])
      else:
        eL = float(eL)
        
      if r > 1.: e = eH
      if r < 1.: e = eL
      Pull = float((1.-r)/abs(e))
      Residuals = float(1.-r)
      hlist_Pull[i].Fill(Pull)
      hlist_RSS[i].Fill(Residuals)
      with open("Output_%s.txt"%ch, "a") as text_file:
        text_file.write("Toy nr:  {}  \n".format(ii))
        text_file.write("r:  {}\n".format(rate))
        text_file.write("Obs. sign.:  {}\n".format(obssig))
        text_file.write("Exp. sign.:  {}\n".format(expsig))
        text_file.write("Pull:  {}\n".format(Pull))
        text_file.write("Residuals:  {}\n".format(Residuals))
        text_file.write("My r:  {}\n".format(r))
        text_file.write("My eH:  {}\n".format(eH))
        text_file.write("My eL:  {}\n".format(eL))
      print ""
      print "" 
      print "Rate r == %f +- %f/%f" %(r,eH,eL)
      print "Pull = %f" %Pull
      print "Residuals = %f" %Residuals
      print "Expected significance == %f" %(ExpSig)
      print "Observed significance == %f" %(ObsSig)
      print ""
      print ""
      # biasname = "%s_bias" %ch
      # pullname = "%s_pull" %ch
      # rssname = "%s_rss" %ch
      # expsigmaname = "%s_ExpSignificance" %ch
      # obssigmaname = "%s_ObsSignificance" %ch
      # hlist_Bias[i].SetName(biasname)
      # hlist_Pull[i].SetName(pullname)
      # hlist_RSS[i].SetName(rssname)
      # hlist_SigmaExp[i].SetName(expsigmaname)
      # hlist_SigmaObs[i].SetName(obssigmaname)
      # cmd = "rm tmp.txt && rm tmp2.txt && rm tmp3.txt && rm roostats*"
      # print cmd
      # os.system(cmd)

pullcanv = []
rsscanv = []
biascanv = []
sigmaexpcanv = []
sigmaobscanv = []

W = 600
H = 700
H_ref = 700
W_ref = 600
T = 0.08*H_ref
B = 0.12*H_ref
L = 0.12*W_ref
R = 0.04*W_ref



l1 = TLatex()
l1.SetNDC()
l1.SetTextAlign(12)
l1.SetTextFont(42)
l1.SetTextSize(0.025)

i = -1
for ch in channels:
  i +=1

  addInfo = TPaveText(0.2147651,0.7842262,0.4832215,0.8900595,"NDC")
  addInfo.SetFillColor(0)
  addInfo.SetLineColor(0)
  addInfo.SetFillStyle(0)
  addInfo.SetBorderSize(0)
  addInfo.SetTextFont(42)
  addInfo.SetTextSize(0.040)
  addInfo.SetTextAlign(12)
  addInfo.AddText("W'(1.4 TeV)#rightarrowWZ (#sigma = 1 pb)")
  addInfo.AddText("%s category"%ch)
  
  c2 = TCanvas("pullCanv%s"%ch,"pullCanv%s"%ch,W,H)
  c2.GetWindowHeight()
  c2.GetWindowWidth()
  c2.SetLeftMargin(0.2)
  c2.SetBottomMargin(0.2)
  
  hlist_Pull[i].SetTitle("")
  hlist_Pull[i].GetXaxis().SetTitle("Nr. toys")
  hlist_Pull[i].GetXaxis().SetTitle("#frac{r_{in}-r_{out}}{#sigma}")
  hlist_Pull[i].GetXaxis().SetTitleSize(0.06)
  hlist_Pull[i].GetXaxis().SetTitleOffset(0.95)
  hlist_Pull[i].GetXaxis().SetLabelSize(0.05)
  hlist_Pull[i].GetYaxis().SetTitleSize(0.06)
  hlist_Pull[i].GetYaxis().SetTitleOffset(1.0)
  hlist_Pull[i].GetYaxis().SetLabelSize(0.05)
  hlist_Pull[i].SetMarkerSize(0.9)
  hlist_Pull[i].SetMarkerStyle(20)
  hlist_Pull[i].SetLineColor(kRed)
  hlist_Pull[i].GetXaxis().SetNdivisions(405)
  hlist_Pull[i].Draw("pe0")
  hlist_Pull[i].GetXaxis().SetRangeUser(hlist_Pull[i].GetXaxis().GetXmin(),hlist_Pull[i].GetXaxis().GetXmax())
  hlist_Pull[i].SetMaximum(hlist_Pull[i].GetMaximum()*2.0)
  l1.DrawLatex(0.2147651,0.70, "Mean: %.2f, RMS: %.2f (%i toys)" %(hlist_Pull[i].GetMean(),hlist_Pull[i].GetRMS(),ntoys))
  addInfo.Draw('same')
  c2.Update()
  pullcanv.append(c2)
  cname = ch + "_pull.pdf"
  c2.SaveAs(cname)

  c1 = TCanvas("bias_%s"%ch,"bias_%s"%ch,W,H)
  c1.GetWindowHeight()
  c1.GetWindowWidth()
  c1.SetLeftMargin(0.2)
  c1.SetBottomMargin(0.2)
  hlist_Bias[i].SetTitle("")
  hlist_Bias[i].GetXaxis().SetTitle("Signal strength r")
  hlist_Bias[i].GetYaxis().SetTitle("Nr. toys")
  hlist_Bias[i].GetXaxis().SetTitleSize(0.06)
  hlist_Bias[i].GetXaxis().SetTitleOffset(0.95)
  hlist_Bias[i].GetXaxis().SetLabelSize(0.05)
  hlist_Bias[i].GetYaxis().SetTitleSize(0.06)
  hlist_Bias[i].GetYaxis().SetTitleOffset(1.0)
  hlist_Bias[i].GetYaxis().SetLabelSize(0.05)
  hlist_Bias[i].SetMarkerSize(0.9)
  hlist_Bias[i].SetMarkerStyle(20)
  hlist_Bias[i].SetLineColor(kRed)
  hlist_Bias[i].GetXaxis().SetNdivisions(405)
  hlist_Bias[i].Draw("pe0")
  hlist_Bias[i].GetXaxis().SetRangeUser(hlist_Bias[i].GetXaxis().GetXmin(),hlist_Bias[i].GetXaxis().GetXmax())
  hlist_Bias[i].SetMaximum(hlist_Bias[i].GetMaximum()*1.5)
  l1.DrawLatex(0.2147651,0.70, "Mean: %.2f, RMS: %.2f (%i toys)" %(hlist_Bias[i].GetMean(),hlist_Bias[i].GetRMS(),ntoys))
  addInfo.Draw('same')
  c1.Update()
  # line = TLine(bias.GetMean(1),0,bias.GetMean(1),bias.GetMaximum())
  # line.SetVertical()
  # line.SetLineColor(kRed)
  # line.SetLineWidth(2)
  # line.Draw("same")
  # c1.Update()
  biascanv.append(c1)
  cname = ch + "_r.pdf"
  c1.SaveAs(cname)


  c3 = TCanvas("sigmaObs_%s"%ch,"sigmaObs_%s"%ch,W,H)
  c3.GetWindowHeight()
  c3.GetWindowWidth()
  c3.SetLeftMargin(0.2)
  c3.SetBottomMargin(0.2)
  hlist_SigmaObs[i].SetTitle("")
  hlist_SigmaObs[i].GetXaxis().SetTitle("Observed significance #sigma")
  hlist_SigmaObs[i].GetYaxis().SetTitle("Nr. toys")
  hlist_SigmaObs[i].GetXaxis().SetTitleSize(0.06)
  hlist_SigmaObs[i].GetXaxis().SetTitleOffset(0.95)
  hlist_SigmaObs[i].GetXaxis().SetLabelSize(0.05)
  hlist_SigmaObs[i].GetYaxis().SetTitleSize(0.06)
  hlist_SigmaObs[i].GetYaxis().SetTitleOffset(1.0)
  hlist_SigmaObs[i].GetYaxis().SetLabelSize(0.05)
  hlist_SigmaObs[i].SetMarkerSize(0.9)
  hlist_SigmaObs[i].SetMarkerStyle(20)
  hlist_SigmaObs[i].SetLineColor(kRed)
  hlist_SigmaObs[i].GetXaxis().SetNdivisions(405)
  hlist_SigmaObs[i].Draw("pe0")
  hlist_SigmaObs[i].GetXaxis().SetRangeUser(hlist_SigmaObs[i].GetXaxis().GetXmin(),hlist_SigmaObs[i].GetXaxis().GetXmax())
  hlist_SigmaObs[i].SetMaximum(hlist_SigmaObs[i].GetMaximum()*1.5)
  l1.DrawLatex(0.2147651,0.70, "Mean: %.2f, RMS: %.2f (%i toys)" %(hlist_SigmaObs[i].GetMean(),hlist_SigmaObs[i].GetRMS(),ntoys))
  addInfo.Draw('same')
  c3.Update()
  # line = TLine(Sigma.GetMean(1),0,Sigma.GetMean(1),Sigma.GetMaximum())
  # line.SetVertical()
  # line.SetLineColor(kRed)
  # line.SetLineWidth(2)
  # line.Draw("same")
  # c3.Update()
  sigmaobscanv.append(c3)
  cname = ch + "_sigmaObs.pdf"
  c3.SaveAs(cname)

  c4 = TCanvas("sigmaExp_%s"%ch,"sigmaExp_%s"%ch,W,H)
  c4.GetWindowHeight()
  c4.GetWindowWidth()
  c4.SetLeftMargin(0.2)
  c4.SetBottomMargin(0.2)
  hlist_SigmaExp[i].SetTitle("")
  hlist_SigmaExp[i].GetXaxis().SetTitle("Expected significance #sigma")
  hlist_SigmaExp[i].GetYaxis().SetTitle("Nr. toys")
  hlist_SigmaExp[i].GetXaxis().SetTitleSize(0.06)
  hlist_SigmaExp[i].GetXaxis().SetTitleOffset(0.95)
  hlist_SigmaExp[i].GetXaxis().SetLabelSize(0.05)
  hlist_SigmaExp[i].GetYaxis().SetTitleSize(0.06)
  hlist_SigmaExp[i].GetYaxis().SetTitleOffset(1.0)
  hlist_SigmaExp[i].GetYaxis().SetLabelSize(0.05)
  hlist_SigmaExp[i].SetMarkerSize(0.9)
  hlist_SigmaExp[i].SetMarkerStyle(20)
  hlist_SigmaExp[i].SetLineColor(kRed)
  hlist_SigmaExp[i].GetXaxis().SetNdivisions(405)
  hlist_SigmaExp[i].Draw("pe0")
  hlist_SigmaExp[i].GetXaxis().SetRangeUser(hlist_SigmaExp[i].GetXaxis().GetXmin(),hlist_SigmaExp[i].GetXaxis().GetXmax())
  hlist_SigmaExp[i].SetMaximum(hlist_SigmaExp[i].GetMaximum()*1.5)
  l1.DrawLatex(0.2147651,0.70, "Mean: %.2f, RMS: %.2f (%i toys)" %(hlist_SigmaExp[i].GetMean(),hlist_SigmaExp[i].GetRMS(),ntoys))
  addInfo.Draw('same')
  c4.Update()
  # line = TLine(Sigma.GetMean(1),0,Sigma.GetMean(1),Sigma.GetMaximum())
  # line.SetVertical()
  # line.SetLineColor(kRed)
  # line.SetLineWidth(2)
  # line.Draw("same")
  # c4.Update()
  sigmaexpcanv.append(c4)
  cname = ch + "_sigmaExp.pdf"
  c4.SaveAs(cname)
  
  c5 = TCanvas("rssCanv_%s"%ch,"rssCanv_%s"%ch,W,H)
  c5.GetWindowHeight()
  c5.GetWindowWidth()
  c5.SetLeftMargin(0.2)
  c5.SetBottomMargin(0.2)
  hlist_RSS[i].SetTitle("")
  hlist_RSS[i].GetXaxis().SetTitle("r_{in}-r_{out}")
  hlist_RSS[i].GetYaxis().SetTitle("Nr. toys")
  hlist_RSS[i].GetXaxis().SetTitleSize(0.06)
  hlist_RSS[i].GetXaxis().SetTitleOffset(0.95)
  hlist_RSS[i].GetXaxis().SetLabelSize(0.05)
  hlist_RSS[i].GetYaxis().SetTitleSize(0.06)
  hlist_RSS[i].GetYaxis().SetTitleOffset(1.0)
  hlist_RSS[i].GetYaxis().SetLabelSize(0.05)
  hlist_RSS[i].SetMarkerSize(0.9)
  hlist_RSS[i].SetLineColor(kRed)
  hlist_RSS[i].SetMarkerStyle(20)
  hlist_RSS[i].GetXaxis().SetNdivisions(405)
  hlist_RSS[i].Draw("pe0")
  hlist_RSS[i].GetXaxis().SetRangeUser(hlist_RSS[i].GetXaxis().GetXmin(),hlist_RSS[i].GetXaxis().GetXmax())
  hlist_RSS[i].SetMaximum(hlist_RSS[i].GetMaximum()*1.5)
  l1.DrawLatex(0.2147651,0.70, "Mean: %.2f, RMS: %.2f (%i toys)" %(hlist_RSS[i].GetMean(),hlist_RSS[i].GetRMS(),ntoys))
  addInfo.Draw('same')
  c5.Update()
  rsscanv.append(c5)
  cname = ch + "_rss.pdf"
  c5.SaveAs(cname)

f = TFile("bias-studies-Wprime%spb.root"%scaleS,'RECREATE')
for i in range (0,len(channels)):
  biascanv[i].Write()
  pullcanv[i].Write()
  rsscanv[i].Write()
  sigmaexpcanv[i].Write()
  sigmaobscanv[i].Write()
  hlist_Bias[i].Write()
  hlist_Pull[i].Write()
  hlist_RSS[i].Write()
  hlist_SigmaExp[i].Write()
  hlist_SigmaObs[i].Write()
for c in range(0,len(mjjcanv)):
  mjjcanv[c].Write()
f.Write()
f.Close()
filetmpS.Close()
del histS
del biascanv
del pullcanv
del mjjcanv
del sigmaexpcanv
del sigmaobscanv
del hlist_Bias
del hlist_Pull
del hlist_RSS
del hlist_SigmaExp
del hlist_SigmaObs