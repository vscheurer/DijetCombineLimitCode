import ROOT as rt
from array import *
import time

rt.gStyle.SetOptFit(1)

massBins =[1, 3, 6, 10, 16, 23, 31, 40, 50, 61, 74, 88, 103, 119, 137, 156, 176, 197, 220, 244, 270, 296, 325, 354, 386, 419, 453, 489, 526, 565, 606, 649, 693, 740, 788, 838, 890, 944, 1000, 1058,
             1118, 1181, 1246, 1313, 1383, 1455, 1530, 1607, 1687, 1770, 1856, 1945, 2037, 2132, 2231, 2332, 2438, 2546, 2659, 2775, 2895, 3019, 3147, 3279, 3416, 3558, 3704, 3854, 4010, 4171, 4337, 
             4509, 4686, 4869, 5058, 5253, 5455, 5663, 5877, 6099, 6328, 6564, 6808]

xbins = array('d',massBins)


fileIN = rt.TFile.Open("input/PseudodataB.root")
histos = ["DijetMassHighPuriZZ", "DijetMassHighPuriWZ", "DijetMassHighPuriWW",
          "DijetMassLowPuriZZ", "DijetMassLowPuriWZ", "DijetMassLowPuriWW"]
histos = ["DijetMassHighPuriWW"]


xsec_Wprime2TeV = 0.027864814398000003
lumi = 2460.0
scaleS=50.

print " Let's go"
        
        
for h in histos:
    title = h.replace("DijetMass","")
    htmp = fileIN.Get(h)
    dataDistOLD = htmp.Rebin(len(xbins),"hMass_rebinned",xbins)
    minVal = 1000.
    maxVal = 3147.

    bins = []
    for i in range(0, dataDistOLD.GetXaxis().GetNbins()):
        thisVal = dataDistOLD.GetXaxis().GetBinLowEdge(i+1)
        if thisVal >= minVal and thisVal < maxVal: bins.append(dataDistOLD.GetXaxis().GetBinLowEdge(i+1))

    dataDist = rt.TH1F("dataDist", "dataDist", len(bins)-1, array('d',bins))
    for i in range(0, dataDistOLD.GetXaxis().GetNbins()):
        binCenter = dataDistOLD.GetXaxis().GetBinCenter(i+1)
        binContent = dataDistOLD.GetBinContent(i+1)
        binWidth = dataDistOLD.GetBinWidth(i+1)
        if binCenter >= minVal:
            iBin =  dataDist.GetXaxis().FindBin(binCenter)
            dataDist.SetBinContent(iBin, binContent)
    
    
    fileS = "/shome/thaarres/EXOVVAnalysisRunII/LimitCode/CMSSW_7_1_5/src/DijetCombineLimitCode/input/WprimeToWZ_13TeV_2000GeV.root "
    infileS = rt.TFile.Open(fileS,"READ")
    hsig = infileS.Get(h)


    sigDist = rt.TH1F("sigDist", "sigDist", len(bins)-1, array('d',bins))
    for i in range(0, hsig.GetXaxis().GetNbins()):
        binCenter = hsig.GetXaxis().GetBinCenter(i+1)
        binContent = hsig.GetBinContent(i+1)
        binWidth = hsig.GetBinWidth(i+1)
        if binCenter >= minVal:
            iBin =  sigDist.GetXaxis().FindBin(binCenter)
            sigDist.SetBinContent(iBin, binContent)
    
    sigDist.Scale(xsec_Wprime2TeV*scaleS*lumi)
    # time.sleep(100)
    # create PDF`
    p0 = rt.RooRealVar("p0", "p0", 8.5 , 0. , 22.)
    p1 = rt.RooRealVar("p1", "p1", 0. , 0. , 22.)
    p2 = rt.RooRealVar("p2", "p2", 6., 2., 12.)
    p3 = rt.RooRealVar("p3", "p3", 0., -12., 12.)
    mjj = rt.RooRealVar("mjjCMS","Dijet invariant mass [GeV]",len(bins)-1, bins[0], bins[-1])
    #bkg_fit = rt.RooGenericPdf("bkg_fitCMS", "@3*10E-6*pow(1-@0/13000., @1)/pow(@0/8000., @2)", rt.RooArgList(mjj, p1, p2,p0))
   
    bkg_fit = rt.RooGenericPdf("bkg_fitCMS", "pow(1-@0/13000., @1)/pow(@0/13000., @2+@3*log(@0))", rt.RooArgList(mjj, p1, p2,p3))
    # p1.setConstant(rt.kTRUE)
    p3.setConstant(rt.kTRUE)
    # dummy signal
    sigfrac = rt.RooRealVar("sigfrac","sigfrac",0.5, 0.3, 1.0)
    alpha = rt.RooRealVar("alpha","alpha",2., 0.5, 3)
    sign = rt.RooRealVar("sign","sign",13.0, 0.5, 10)
    mean = rt.RooRealVar("mean","mean",2000., 0., 2000.)
    sigma = rt.RooRealVar("sigma","sigma",100., 0., 100.)
    gauss = rt.RooGaussian("gauss", "gauss", mjj, mean, sigma)
    cb    = rt.RooCBShape("cb", "cb",mjj, mean, sigma, alpha, sign)
    sig_fit = rt.RooAddPdf("sigP", "sigP",gauss, cb, sigfrac)
    datasetSIG = rt.RooDataHist("sig", "sig", rt.RooArgList(mjj), rt.RooFit.Import(sigDist))
    sig_fit.fitTo(datasetSIG,rt.RooFit.Save(), rt.RooFit.Range(2000*0.5,2000*1.5),rt.RooFit.SumW2Error(rt.kTRUE))
   
    sigfrac.setConstant(rt.kTRUE)
    alpha.setConstant(rt.kTRUE)
    sign.setConstant(rt.kTRUE)
    mean.setConstant(rt.kTRUE)
    sigma.setConstant(rt.kTRUE)
    
    
    # signal and background yields
    s = rt.RooRealVar("Ns", "signal yield", 0.000000001, 0, 1000)
    b = rt.RooRealVar("Nb", "background yield", dataDist.Integral(), 0, dataDist.Integral()*10.)
    # extended PDF
    sumPDF = rt.RooAddPdf("sum", "gaussian plus exponential PDF",
                          rt.RooArgList(sig_fit, bkg_fit), rt.RooArgList(s, b))
    # fix the signal to 0 
    # sigma.setConstant(rt.kTRUE)
   #  mean.setConstant(rt.kTRUE)
    # s.setConstant(rt.kTRUE)
   #  alpha.setConstant(rt.kTRUE)
   #  sign.setConstant(rt.kTRUE)
   #  sigfrac.setConstant(rt.kTRUE)

    mjjbins = rt.RooBinning(len(bins)-1, array('d',bins), "mjjbins")
    mjj.setBinning(mjjbins)

    ws = rt.RooWorkspace("ws","ws")
    getattr(ws,'import')(p0)
    getattr(ws,'import')(p1)
    getattr(ws,'import')(p2)
    getattr(ws,'import')(mjj)
    getattr(ws,'import')(mean)
    getattr(ws,'import')(sigma)
    getattr(ws,'import')(s)
    getattr(ws,'import')(b)
    getattr(ws,'import')(sign)
    getattr(ws,'import')(alpha)
    getattr(ws,'import')(sigfrac)
    getattr(ws,'import')(sumPDF)
  
    
    

    # create RooDataHist
    dataset = rt.RooDataHist("dataCMS", "dataCMS", rt.RooArgList(mjj), rt.RooFit.Import(dataDist))
    getattr(ws,'import')(dataset)

    # fit the distribution
    #fr = bkg_fit.fitTo(dataset,rt.RooFit.Save(), rt.RooFit.SumW2Error(rt.kTRUE), rt.RooFit.Extended(rt.kFALSE))
    
    fr = sumPDF.fitTo(dataset,rt.RooFit.Save(), rt.RooFit.SumW2Error(rt.kTRUE))
    
    

    # draw the fit result
    frame = mjj.frame()
    
    dataset.plotOn(frame,rt.RooFit.DataError(rt.RooAbsData.SumW2), rt.RooFit.Binning(mjjbins))
    # bkg_fit.plotOn(frame, rt.RooFit.VisualizeError(fr), rt.RooFit.FillColor(rt.kBlue-10))
    dataset.plotOn(frame,rt.RooFit.DataError(rt.RooAbsData.SumW2), rt.RooFit.Binning(mjjbins))
    # bkg_fit.plotOn(frame,rt.RooFit.LineColor(rt.kBlue+1),rt.RooFit.Name("bkg"))

    # sig_fit.plotOn(frame,rt.RooFit.LineColor(rt.kRed),rt.RooFit.Name("sig"))
    
    sumPDF.plotOn(frame,rt.RooFit.Components("sigP"),rt.RooFit.LineColor(rt.kRed),rt.RooFit.Name("sig"), rt.RooFit.LineStyle(rt.kDashed))
    sumPDF.plotOn(frame,rt.RooFit.Components("bkg_fitCMS"),rt.RooFit.LineColor(rt.kBlue),rt.RooFit.Name("bkg"),rt.RooFit.LineStyle(rt.kDashed))
    sumPDF.plotOn(frame,rt.RooFit.LineColor(rt.kBlack),rt.RooFit.Name("sumPDF"))
    frame3 = mjj.frame()
    hpull = frame.pullHist()
    frame3.addPlotable(hpull,"P")
    
    c1 =rt.TCanvas("c1","",800,800)
    c1.SetLogy()
    c1.Divide(1,2,0,0,0)
    c1.SetLogy()
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
    
    frame.SetMinimum(0.1)
    frame.SetMaximum(1E3)
    frame.SetName("mjjFit")
    frame.GetYaxis().SetName("#frac{dN}{dm_{jj}} [GeV^{-1}]")
    frame.SetTitle("")
    frame.Draw()
    
    legend = rt.TLegend(0.500351,0.7177579,0.6390289,0.9182374)
    legend.SetTextSize(0.038)
    legend.SetLineColor(0)
    legend.SetShadowColor(0)
    legend.SetLineStyle(1)
    legend.SetLineWidth(1)
    legend.SetFillColor(0)
    legend.SetFillStyle(0)
    legend.SetMargin(0.35)
    legend.AddEntry(frame.findObject("bkg"),"Background comp.","l")
    legend.AddEntry(frame.findObject("sig"),"Signal comp.","l")
    legend.AddEntry(frame.findObject("sumPDF"),"Sum PDF","l")
    legend.Draw("same")
    
    c1.cd(2)
    p11_2 = c1.GetPad(2)
    p11_2.SetPad(0.01,0.02,0.99,0.27)
    p11_2.SetBottomMargin(0.35)
    p11_2.SetRightMargin(0.05)
    p11_2.SetGridx()
    p11_2.SetGridy()
    frame3.SetMinimum(-3.2)
    frame3.SetMaximum(3.2)
    frame3.SetTitle("")
    frame3.SetXTitle("Dijet invariant mass [GeV]")
    frame3.GetXaxis().SetTitleSize(0.06)
    frame3.SetYTitle("#frac{Data-Fit}{#sigma}")
    frame3.GetYaxis().SetTitleSize(0.15)
    frame3.GetYaxis().SetTitleOffset(0.20)
    frame3.GetYaxis().SetLabelSize(0.09)
    frame3.GetXaxis().SetTitleSize(0.15)
    frame3.GetXaxis().SetTitleOffset(0.90)
    frame3.GetXaxis().SetLabelSize(0.12)
    frame3.GetXaxis().SetNdivisions(405)

    frame3.Draw("same")
    line = rt.TLine(minVal,0,maxVal,0)
    line.Draw("same")
    
    print title
    c1.SaveAs("%s_fitTodata_noSig_3par.pdf"%title)

     # write the output
    fileOut = rt.TFile.Open("fitTodata_noSig_3par%s.root" %title,"recreate")
    dataDist.Write()
    c1.Write()
    sumPDF.Write()
    bkg_fit.Write()
    fileOut.Close()
    
    time.sleep(100)
    #
    # dataNLL = fr.minNll()
    #
    # print "LIKELIHOOD ON DATA: %f" %dataNLL

    # # generate 1k toyMCs
   #  mcstudy = rt.RooMCStudy(sumPDF,rt.RooArgSet(mjj),rt.RooFit.Binned(rt.kTRUE),rt.RooFit.Silence(),rt.RooFit.Extended(),
   #                          rt.RooFit.FitOptions(rt.RooFit.Save(rt.kTRUE),rt.RooFit.PrintEvalErrors(0)))
   #  mcstudy.generateAndFit(1000)
   #  plotNLL = mcstudy.plotNLL(rt.RooFit.Bins(100))
   #
   #  c2 = rt.TCanvas("c2","c2", 600, 600)
   #  plotNLL.SetTitle(title)
   #  plotNLL.Draw()
   #
   #  graph = rt.TGraph(2, array('d',[dataNLL,dataNLL]), array('d',[0.0, plotNLL.GetMaximum()]))
   #  graph.SetLineStyle(2)
   #  graph.SetLineColor(2)
   #  graph.Draw("same")
   #
   #  stepy = 0.1*plotNLL.GetMaximum()
   #  stepx = (plotNLL.GetXaxis().GetXmax()-plotNLL.GetXaxis().GetXmin())/10.
   #
   #  graph2 = rt.TGraph(3, array('d',[dataNLL-stepx,dataNLL,dataNLL+stepx]), array('d',[stepy,0.0, stepy]))
   #  graph2.SetLineStyle(2)
   #  graph2.SetLineColor(2)
   #  graph2.Draw("same")
   #
   #  c2.SaveAs("gof_%s.png" %title)
   #
   #  # write the output
   #  fileOut = rt.TFile.Open("gof_%s.root" %title,"recreate")
   #  dataDist.Write()
   #  c1.Write()
   #  c2.Write()
   #  plotNLL.Write()
   #  fileOut.Close()
   #
