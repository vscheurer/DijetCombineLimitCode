from ROOT import *
import numpy as np

Limits = np.genfromtxt("/usr/users/vscheurer/meanvalues_0.3.txt")
WQRange = ['']
mass = [1.4,1.6,1.8,2,2.5,3,3.5,4]

WQLine1 = TGraphErrors(1)
WQLine1.SetMarkerColor(8)
WQLine1.SetMarkerStyle(21)


WQLine09 = TGraphErrors(1)
WQLine09.SetMarkerColor(17)
WQLine09.SetMarkerStyle(21)
WQLine09.SetLineColor(17)
WQLine09.SetLineWidth(2)

WQLine08 = TGraphErrors(1)
WQLine08.SetMarkerColor(6)
WQLine08.SetMarkerStyle(21)
WQLine08.SetLineColor(6)
WQLine08.SetLineWidth(2)

WQLine06 = TGraphErrors(1)
WQLine06.SetMarkerColor(9)
WQLine06.SetMarkerStyle(21)
WQLine06.SetLineColor(9)
WQLine06.SetLineWidth(2)

WQLine03 = TGraphErrors(1)
WQLine03.SetMarkerColor(2)
WQLine03.SetMarkerStyle(21)
WQLine03.SetLineColor(2)
WQLine03.SetLineWidth(2)

FiveSigma = TGraphErrors(1)
FiveSigma.SetLineColor(7)
FiveSigma.SetLineWidth(2)
FiveSigma.SetLineStyle(1)


for WQ in WQRange:
    for m in range(len(mass)):
       file1 = TFile.Open('datacardsFakeSignal0.9/higgsCombineFakeSignal'+str(WQ)+'.ProfileLikelihood.mH'+str(int(mass[m]*1000))+'.root','READ')
       file2 = TFile.Open('datacardsFakeSignal0.8/higgsCombineFakeSignal'+str(WQ)+'.ProfileLikelihood.mH'+str(int(mass[m]*1000))+'.root','READ')
       file3 = TFile.Open('datacardsFakeSignal0.6/higgsCombineFakeSignal'+str(WQ)+'.ProfileLikelihood.mH'+str(int(mass[m]*1000))+'.root','READ')
       file4 = TFile.Open('datacardsFakeSignal0.3/higgsCombineFakeSignal'+str(WQ)+'.ProfileLikelihood.mH'+str(int(mass[m]*1000))+'.root','READ')
       file5 = TFile.Open('datacardsFakeSignal_wb/higgsCombineFakeSignal'+str(WQ)+'.ProfileLikelihood.mH'+str(int(mass[m]*1000))+'.root','READ')
       Tree1 = file1.Get("limit")
       Tree2 = file2.Get("limit")
       Tree3 = file3.Get("limit")
       Tree4 = file4.Get("limit")
       Tree5 = file5.Get("limit")
       for entry in Tree1:
            Signi09 = entry.limit
       for entry in Tree2:
            Signi08 = entry.limit
       for entry in Tree3:
            Signi06 = entry.limit
       for entry in Tree4:
            Signi03 = entry.limit
       for entry in Tree5:
            Signi_wb = entry.limit
       FiveSigma.SetPoint(m,mass[m],5)     
       WQLine1.SetPoint(m,mass[m],Signi_wb)
       WQLine09.SetPoint(m,mass[m],(Signi09/Signi_wb)-1)
       WQLine08.SetPoint(m,mass[m],(Signi08/Signi_wb)-1)
       WQLine06.SetPoint(m,mass[m],(Signi06/Signi_wb)-1)
       WQLine03.SetPoint(m,mass[m],(Signi03/Signi_wb)-1)

c1 = TCanvas("c1","c1",50,50,640,480)
gStyle.SetLegendBorderSize(0)
l = TLegend(0.654361,0.723552,0.98124,0.9024967)
#l.AddEntry(WQLine1,"without btags")
#l.AddEntry(WQLine09,"btag 0.9")
l.AddEntry(WQLine08,"btag 0.8")
l.AddEntry(WQLine06,"btag 0.6")
#l.AddEntry(WQLine03,"btag 0.3")
l.SetFillStyle(0)


#WQLine08.GetYaxis().SetRangeUser(0,10)
WQLine08.Draw("Al")
#FiveSigma.Draw("SAME l")
#WQLine1.Draw("SAME p")
WQLine09.Draw("SAME l")
WQLine06.Draw("SAME l")
#WQLine03.Draw("SAME l")
l.Draw("SAME")
#higgsCombineFakeSignal05.ProfileLikelihood.mH1600
x = raw_input()
