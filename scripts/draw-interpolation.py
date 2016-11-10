from ROOT import *
import ROOT as rt
from array import *
import time
import CMS_lumi, tdrstyle
import sys
import operator

tdrstyle.setTDRStyle()

CMS_lumi.lumi_13TeV = ""
CMS_lumi.writeExtraText = 1
CMS_lumi.extraText = "Simulation Preliminary"
CMS_lumi.lumi_sqrtS = "13 TeV" # used with iPeriod = 0, e.g. for simulation-only plots (default is an empty string)
iPos = 0
if( iPos==0 ): CMS_lumi.relPosX = 0.12
iPeriod = 4

H_ref = 600; 
W_ref = 800; 
W = W_ref
H  = H_ref

T = 0.08*H_ref
B = 0.12*H_ref 
L = 0.12*W_ref
R = 0.04*W_ref

col = TColor()

rebin = 10
models = ["BulkWW","BulkZZ","ZprimeWW","WprimeWZ"]
histnames = [#'DijetMassHighPuriVV',
            'DijetMassHighPuriWW',
            'DijetMassHighPuriWZ',
            'DijetMassHighPuriZZ',
            # 'DijetMassLowPuriVV',
            'DijetMassLowPuriWW',
            'DijetMassLowPuriWZ',
            'DijetMassLowPuriZZ'
            ]

models = ["QstarQZ","QstarQW"]
histnames = [#'DijetMassHighPuriqW',
            'DijetMassHighPuriqZ'
           # 'DijetMassLowPuriqW',
           # 'DijetMassLowPuriqZ'
            ]

                        
for model in models:
  if model.find("ZZ")  !=-1: histnames = ['DijetMassHighPuriZZ']
  if model.find("WZ")  !=-1: histnames = ['DijetMassHighPuriWZ']
  if model.find("WW")  !=-1: histnames = ['DijetMassHighPuriWW']
  if model.find("QZ")  !=-1: histnames = ['DijetMassHighPuriqZ']
  if model.find("QW")  !=-1: histnames = ['DijetMassHighPuriqW']
  masses =[m*100 for m in range(11,42+1)]
  if model.find("BulkZZ") !=-1:  masses =[m*100 for m in range(11,40+1)]
  if model.find("Qstar")  !=-1:  masses =[m*100 for m in range(12,62+1)]
  # masses = [4000]
  bins = []
  ws = rt.RooWorkspace("ws","ws")
  
  yielddict = {}
  maxvaldict = {}
  histlist = []  
  sigfits = []
  for h in histnames:     
    histlist.append([])
    sigfits.append([])
  print "Length of histlist = " , len(histlist)
  for mass in masses:
    print "Working on masspoint " ,mass
    fname = "/shome/thaarres/EXOVVAnalysisRunII/LimitCode/CMSSW_7_1_5/src/DijetCombineLimitCode/input/"+model+"_13TeV_OUT%iGeV.root"%mass
    print "Opening " ,fname
    infile = TFile.Open(fname,"READ")

    for j in range(0,len(histnames)):
      histname = histnames[j]
      print "Getting histogram " ,histname
      hist = infile.Get(histname)
      print "With mean = %i" %hist.GetMean()
      hist.SetName(histname+"%i"%mass+model)
       # hist.Scale(1./hist.Integral())
      hist.Rebin(rebin)
      
      for i in range(0, hist.GetXaxis().GetNbins()):
          thisVal = hist.GetXaxis().GetBinLowEdge(i+1)
          bins.append(hist.GetXaxis().GetBinLowEdge(i+1))
      mjj = rt.RooRealVar("mjjCMS","Dijet invariant mass [GeV]",len(bins)-1, bins[0], bins[-1])
      print "Nr bins = %i  First bin 0 = %i  Last bin = %i" %(len(bins)-1,bins[0], bins[-1])
      mjjbins = rt.RooBinning(len(bins)-1, array('d',bins), "mjjbins")
      mjj.setBinning(mjjbins)
       
      alpha = rt.RooRealVar("alpha%i%s"%(mass,histname),"alpha",1.85288, 0.5, 20)
      mean            = rt.RooRealVar("mean%i%s"%(mass,histname),"mean",mass, mass*0.8, mass*1.2)
      gmean           = rt.RooRealVar("gmean%i%s"%(mass,histname),"gmean", mass*0.8, mass*1.2)
      sigma           = rt.RooRealVar("sigma%i%s"%(mass,histname),"sigma",mass*0.05, 20., 700.)  
      scalesigma_gaus = rt.RooRealVar("scalegsigma%i%s"%(mass,histname),"scalegsigma",2.,1.2,10.)
      sigfrac         = rt.RooRealVar("sigfrac%i%s"%(mass,histname),"sigfrac",0.0, 0.0, 0.25)
      gsigma  = rt.RooFormulaVar("gsigma%i%s"%(mass,histname),"@0*@1", rt.RooArgList(sigma,scalesigma_gaus))
      sign = rt.RooRealVar("sign%i%s"%(mass,histname),"sign",129.697, 0., 300)
      
      gauss   = rt.RooGaussian("gauss%i%s"%(mass,histname), "gauss", mjj, mean, gsigma)
      cb      = rt.RooCBShape( "cb%i%s"   %(mass,histname), "cb"   , mjj, mean, sigma, alpha, sign)
      sig_fit = rt.RooAddPdf(  "sigP%i%s" %(mass,histname), "sigP" , gauss, cb, sigfrac)
          

      # create RooDataHist
      dataset = rt.RooDataHist("dataCMS%i%s"%(mass,histname), "dataCMS", rt.RooArgList(mjj), rt.RooFit.Import(hist))
      print "HELLO!!!!"
      # s = rt.RooRealVar("Ns%i%s"%(mass,histname), "signal yield",hist.Integral( ))
      s = rt.RooRealVar("Ns%i%s"%(mass,histname), "signal yield",hist.Integral( hist.GetXaxis().FindBin(mass*0.8),hist.GetXaxis().FindBin(mass*1.2) ))
      # s = rt.RooRealVar("Ns%i%s"%(mass,histname), "signal yield",dataset.sumEntries())
      yielddict["Ns%i%s"%(mass,histname)]  = int( dataset.sumEntries() )
      maxvaldict["Ns%i%s"%(mass,histname)] = int( hist.GetBinContent( hist.FindBin(hist.GetMean()) ) )
      s.setConstant(rt.kTRUE)
      extPdf=rt.RooExtendPdf("esig%i%s"%(mass,histname),"extended signal p.d.f",sig_fit,s)

      getattr(ws,'import')(mjj)
      getattr(ws,'import')(dataset)
      getattr(ws,'import')(s)
      getattr(ws,'import')(extPdf)
      getattr(ws,'import')(mean)
      getattr(ws,'import')(sigma)
      getattr(ws,'import')(sign)
      getattr(ws,'import')(alpha)
      getattr(ws,'import')(sigfrac)
      getattr(ws,'import')(sig_fit)
      # getattr(ws,'import')(mjj)
#       getattr(ws,'import')(dataset)
#       getattr(ws,'import')(s)
#       getattr(ws,'import')(extPdf)

      # fr = extPdf.fitTo(dataset,rt.RooFit.Save(),rt.RooFit.Range(0.8*mass,1.2*mass),rt.RooFit.SumW2Error(kTRUE))
      fr = extPdf.fitTo(dataset,rt.RooFit.Save(), rt.RooFit.SumW2Error(rt.kTRUE),rt.RooFit.PrintEvalErrors(-1),rt.RooFit.Range(0.8*mass,1.2*mass),rt.RooFit.Extended(kTRUE))
      
         
      sigfits[j].append(extPdf)
      histlist[j].append(hist)
      
            
            
      
      

  # references for T, B, L, R
  T = 0.08*H_ref
  B = 0.12*H_ref 
  L = 0.12*W_ref
  R = 0.04*W_ref
  
  for cat in range(0,len(histnames)):
    
    print "";print "THESE FILES HAVE MEAN/INTEGRAL == 0!! PLEASE DEBUG!!";print "";
    for x in yielddict:
      if yielddict[x] == 0:
        print "Integral = 0: %s = %s" %(x, yielddict[x])
    for x in maxvaldict:
      if maxvaldict[x] == 0:
        print "Mean     = 0:%s = %s" %(x, maxvaldict[x])
    print "";print "";
    
    l = TLegend(0.6243719,0.6241259,0.8253769,0.7604895)
    l.SetTextSize(0.04)
    l.SetLineColor(0)
    l.SetShadowColor(0)
    l.SetLineStyle(1)
    l.SetLineWidth(1)
    l.SetFillColor(0)
    l.SetFillStyle(0)
    l.SetMargin(0.35)
     
    addInfo = TPaveText(0.7650754,0.7937063,0.9547739,0.9143357,"NDC")
    addInfo.SetFillColor(0)
    addInfo.SetLineColor(0)
    addInfo.SetFillStyle(0)
    addInfo.SetBorderSize(0)
    # addInfo.SetTextFont(42)
    addInfo.SetTextSize(0.045)
    addInfo.SetTextAlign(12)
    
    txt = "G_{B}#rightarrow"
    if model.find("Wprime")!=-1:  txt = "W'#rightarrow"
    if model.find("Zprime")!=-1:  txt = "Z'#rightarrow"
    if model.find("Qstar") !=-1:  txt = "q*#rightarrow"
    if model.find("WW")!=-1: txt += "WW"
    if model.find("ZZ")!=-1: txt += "ZZ"
    if model.find("WZ")!=-1: txt += "WZ"
    if model.find("QZ")!=-1: txt += "qZ"
    if model.find("QW")!=-1: txt += "qW"
    addInfo.AddText(txt)
    txt = histnames[cat].split("i")[-1]
    print txt
    if histnames[cat].find("High")!=-1: txt += " HP"
    if histnames[cat].find("Low")!=-1: txt += " LP"
    addInfo.AddText(txt)

    canv = TCanvas("c%i"%cat,"c%i"%cat,50,50,W,H)
    canv.SetFillColor(0)
    canv.SetBorderMode(0)
    canv.SetFrameFillStyle(0)
    canv.SetFrameBorderMode(0)
    canv.SetTickx(0)
    canv.SetTicky(0)
    canv.SetLeftMargin( L/W )
    canv.SetRightMargin( R/W )
    canv.SetTopMargin( T/H )
    canv.SetBottomMargin( B/H )

    
    frame = ws.var("mjjCMS").frame()

    for h in range (0,len(masses)):
      mjj.setRange("sigRegion_%s"%masses[h],masses[h]*0.8,masses[h]*1.2)
      ws.pdf("esig%i%s"%(masses[h],histnames[cat])).plotOn(frame,rt.RooFit.LineColor(col.GetColor("#A43820")),rt.RooFit.Name("sumPDF%i%s"%(masses[h],histnames[cat])),rt.RooFit.Normalization(1, rt.RooAbsReal.RelativeExpected),rt.RooFit.PrintEvalErrors(-1),rt.RooFit.Range("sigRegion_%s"%masses[h]))
      # ws.pdf("esig%i%s"%(masses[h],histnames[cat])).plotOn(frame,rt.RooFit.Components( "gauss%i%s"%(masses[h],histnames[cat]) ),rt.RooFit.LineColor(col.GetColor("#99d8c9")),rt.RooFit.LineStyle(rt.kDashed),rt.RooFit.Name("sumPDFGauss%i%s"%(masses[h],histnames[cat])),rt.RooFit.Normalization(1, rt.RooAbsReal.RelativeExpected),rt.RooFit.PrintEvalErrors(-1),rt.RooFit.Range("sigRegion_%s"%masses[h]))
      # ws.pdf("esig%i%s"%(masses[h],histnames[cat])).plotOn(frame,rt.RooFit.Components( "cb%i%s"   %(masses[h],histnames[cat]) ),rt.RooFit.LineColor(col.GetColor("#fdbb84")),rt.RooFit.Name("sumPDFCB%i%s"%(masses[h],histnames[cat])),rt.RooFit.Normalization(1, rt.RooAbsReal.RelativeExpected),rt.RooFit.PrintEvalErrors(-1),rt.RooFit.Range("sigRegion_%s"%masses[h]))
    frame.SetTitle("")
    frame.Draw()
    frame.GetXaxis().SetRangeUser(955.,4900.)
    if model.find("Qstar")!=-1: frame.GetXaxis().SetRangeUser(990.,6900.)
    nmax = max(maxvaldict.iteritems(), key=operator.itemgetter(1))[0]
    print nmax
    # frame.SetMaximum( 600)
    frame.GetYaxis().SetTitle("Relative yield")
    frame.GetXaxis().SetNdivisions(405)
    frame.GetYaxis().SetNdivisions(905)
    frame.GetXaxis().SetTitle("M_{jj}")
    frame.GetYaxis().SetTitleSize(0.05)
    frame.GetXaxis().SetTitleSize(0.05)
    frame.GetYaxis().SetTitleOffset(1.2)
    frame.GetXaxis().SetTitleOffset(1.1)
    CMS_lumi.CMS_lumi(canv, iPeriod, iPos)
    

    l.AddEntry( frame.findObject("sumPDF%i%s"%(masses[0],histnames[cat])),"Signal PDF", "l" )
    # l.AddEntry( frame.findObject("sumPDFCB%i%s"%(masses[0],histnames[cat])),"Crystal Ball comp.", "l" )
    # l.AddEntry( frame.findObject("sumPDFGauss%i%s"%(masses[0],histnames[cat])),"Gaussian comp.", "l" )
    l.Draw("same")
    addInfo.Draw("same")
    canv.RedrawAxis()
    canv.Update()
    cname = "80X/SigInterpol/interpolation_%s_%s.pdf" %(model,histnames[cat])
    canv.SaveAs(cname)
    canv.SaveAs(cname.replace(".pdf",".root"))
    cname = "/mnt/t3nfs01/data01/shome/thaarres/Notes/notes/AN-16-235/trunk/plots/interpolation_%s_%s.pdf" %(model,histnames[cat])
    canv.SaveAs(cname)
    canv.SaveAs(cname.replace(".pdf",".root"))
  time.sleep(5)
  
     

  
              