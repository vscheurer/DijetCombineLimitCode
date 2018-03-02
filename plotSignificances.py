from ROOT import *
import numpy as np

Limits = np.genfromtxt("/usr/users/vscheurer/meanvalues_0.3.txt")
#WQRange = ['','07','05']
WQRange = ['']
mass = [1.4,1.6,1.8,2,2.5,3,3.5,4]

WQLine1 = TGraphErrors(1)
WQLine1.SetMarkerColor(8)
WQLine1.SetMarkerStyle(21)


#WQLine09 = TGraphErrors(1)
#WQLine09.SetMarkerColor(17)
#WQLine09.SetMarkerStyle(21)


#WQLine08 = TGraphErrors(1)
#WQLine08.SetMarkerColor(6)
#WQLine08.SetMarkerStyle(21)


WQLine07 = TGraphErrors(1)
WQLine07.SetMarkerColor(9)
WQLine07.SetMarkerStyle(21)


WQLine05 = TGraphErrors(1)
WQLine05.SetMarkerColor(2)
WQLine05.SetMarkerStyle(21)

FiveSigma = TGraphErrors(1)
FiveSigma.SetLineColor(7)
FiveSigma.SetLineWidth(2)
FiveSigma.SetLineStyle(1)


for WQ in WQRange:
    for m in range(len(mass)):
       file = TFile.Open('datacardsFakeSignal_wb/higgsCombineFakeSignal'+str(WQ)+'.ProfileLikelihood.mH'+str(int(mass[m]*1000))+'.root','READ')
       Tree = file.Get("limit")
       for entry in Tree:
            Signi = entry.limit
       FiveSigma.SetPoint(m,mass[m],5)     
       if WQ == '':  WQLine1.SetPoint(m,mass[m],Signi)
       #if WQ == '09':  WQLine09.SetPoint(m,mass[m],Signi)
       #if WQ == '08':  WQLine08.SetPoint(m,mass[m],Signi)
       if WQ == '07':  WQLine07.SetPoint(m,mass[m],Signi)
       if WQ == '05':  WQLine05.SetPoint(m,mass[m],Signi)

c1 = TCanvas("c1","c1",50,50,640,480)
gStyle.SetLegendBorderSize(0)
l = TLegend(0.654361,0.723552,0.98124,0.9024967)
l.AddEntry(WQLine1,"cross Section at expected limit")
#l.AddEntry(WQLine09,"0.9*expected limit")
#l.AddEntry(WQLine08,"0.8*expected limit")
l.AddEntry(WQLine07,"0.7*expected limit")
l.AddEntry(WQLine05,"0.5*expected limit")
l.SetFillStyle(0)


WQLine1.GetYaxis().SetRangeUser(0,10)
WQLine1.Draw("AP")
FiveSigma.Draw("SAME l")
#WQLine09.Draw("SAME p")
#WQLine08.Draw("SAME p")
WQLine07.Draw("SAME p")
WQLine05.Draw("SAME p")
l.Draw("SAME")
#higgsCombineFakeSignal05.ProfileLikelihood.mH1600
x = raw_input()
