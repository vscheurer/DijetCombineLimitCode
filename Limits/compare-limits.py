import ROOT as rt
from ROOT import *
import time
import tdrstyle, CMS_lumi

tdrstyle.setTDRStyle()
CMS_lumi.lumi_13TeV = "12.9 fb^{-1}"
CMS_lumi.writeExtraText = 1
CMS_lumi.extraText = "Preliminary"
CMS_lumi.lumi_sqrtS = "13 TeV" # used with iPeriod = 0, e.g. for simulation-only plots (default is an empty string)
iPos = 11
if( iPos==0 ): CMS_lumi.relPosX = 0.12
iPeriod=4

H_ref = 600; 
W_ref = 800; 
W = W_ref
H  = H_ref

T = 0.08*H_ref
B = 0.12*H_ref 
L = 0.12*W_ref
R = 0.04*W_ref

c1 = rt.TCanvas("c1","c1",50,50,W,H)
c1.SetFillColor(0)
c1.SetBorderMode(0)
c1.SetFrameFillStyle(0)
c1.SetFrameBorderMode(0)
c1.SetLeftMargin( L/W )
c1.SetRightMargin( R/W )
c1.SetTopMargin( T/H )
c1.SetBottomMargin( B/H )
c1.SetTickx(0)
c1.SetTicky(0)
c1.GetWindowHeight()
c1.GetWindowWidth()
c1.SetGrid()
c1.SetLogy()
c1.cd()

frame = c1.DrawFrame(1.0,0.002, 4.0, 0.5)

leg = rt.TLegend(0.598995,0.6902591,0.9446734,0.889011917)
leg.SetTextSize(0.028)
leg.SetLineColor(1)
leg.SetShadowColor(0)
leg.SetLineStyle(1)
leg.SetLineWidth(1)
leg.SetFillColor(kWhite)
leg.SetMargin(0.35)
leg.SetBorderSize(1)

mg = rt.TMultiGraph()
mg.SetTitle("X -> ZZ")

fJES     = rt.TFile.Open("2016/AddedJESJMS/graphs.root"    , "READ")
fJESXSEC = rt.TFile.Open("2016/AddedJESJMSXSEC/graphs.root", "READ")
fNoSys   = rt.TFile.Open("2016/noAddedSys/graphs.root"     , "READ")

grJES     = rt.TGraphErrors(fJES.Get("grmean"))
grJESXSEC = rt.TGraphErrors(fJESXSEC.Get("grmean"))
grNoSys   = rt.TGraphErrors(fNoSys.Get("grmean"))


grJES     .SetLineStyle(1)
grJESXSEC .SetLineStyle(1)
grNoSys   .SetLineStyle(1)
grJES     .SetLineColor(rt.kRed)
grJESXSEC .SetLineColor(rt.kBlue)
grNoSys   .SetLineColor(rt.kBlack)
grJES    .SetLineWidth(2)
grJESXSEC.SetLineWidth(2)
grNoSys  .SetLineWidth(2)

leg.AddEntry(grNoSys   , "No sys.", "Lp")
leg.AddEntry(grJES     , "With JES/JMS sys.", "Lp")
leg.AddEntry(grJESXSEC , "With JES/JMS/Xsec sys.", "Lp")


mg.Add(grJES)
mg.Add(grJESXSEC)
mg.Add(grNoSys)
mg.Draw()
mg.GetYaxis().SetTitle("#sigma #times B(G_{Bulk} #rightarrow WW"") (pb)")
mg.GetXaxis().SetTitle("Resonance mass (TeV)")
mg.GetYaxis().SetTitleSize(0.06)
mg.GetXaxis().SetTitleSize(0.06)
mg.GetXaxis().SetLabelSize(0.045)
mg.GetYaxis().SetLabelSize(0.045)
mg.GetYaxis().SetTitleOffset(0.90)
mg.GetYaxis().CenterTitle(True)
mg.GetXaxis().SetTitleOffset(0.90)
mg.GetXaxis().CenterTitle(True)
mg.GetXaxis().SetNdivisions(508)
mg.GetXaxis().SetLimits(1.0,4.0)
leg.Draw("same")

CMS_lumi.CMS_lumi(c1, iPeriod, iPos)
c1.Update()

c1.SaveAs("2016/compareLimits.root")
c1.SaveAs("2016/compareLimits.pdf" )


time.sleep(300)
