import ROOT as rt
import time



H_ref = 600
W_ref = 800 
W = W_ref
H  = H_ref

T = 0.08*H_ref
B = 0.12*H_ref 
L = 0.12*W_ref
R = 0.04*W_ref    
    
signals = ["ZprimeWW","WZ","BulkWW","BulkZZ"]

def get_xsec_unc(mass):
   uncs = {}
   fin = rt.TFile.Open("/mnt/t3nfs01/data01/shome/thaarres/EXOVVAnalysisRunII/LimitCode/CMSSW_7_1_5/src/DijetCombineLimitCode/EXOVVSystematics/xsec-unc-13TeV.root",'READ')   
   cin = fin.Get('c')
   for p in cin.GetListOfPrimitives():
    if p.InheritsFrom("TMultiGraph"):
     for g in p.GetListOfGraphs(): uncs[g.GetName()] = g.Eval(mass) 
   fin.Close() 
   return uncs
   
   
for signal in signals:

  mg = rt.TMultiGraph()
  mg.SetTitle("X -> ZZ")
  
  print "For signal " ,signal
  gtheory = rt.TGraphErrors(1)
  gtheory.SetLineColor(rt.kRed)
  gtheory.SetLineWidth(3)
  gtheoryUP = rt.TGraphErrors(1)
  gtheoryUP.SetLineColor(rt.kRed-2)
  gtheoryUP.SetLineWidth(3)
  gtheoryDOWN = rt.TGraphErrors(1)
  gtheoryDOWN.SetLineColor(rt.kRed-2)
  gtheoryDOWN.SetLineWidth(3)
  ftheory=open("signalcrosssections13TeV.txt")
  j=0
  radmasses = []
  ymin = []
  ymax = []
  for lines in ftheory.readlines():
   for line in lines.split("\r"):
     if signal in line:
      split=line.split(":")
      
      # print "Mass = %i   Crossection = %f" %(int(split[0][-4:]),float(split[1]))
      
      xsecUnc      =  get_xsec_unc(int(split[0][-4:]))
      pdf_Wprime   = 1+xsecUnc['qq_PDF_Wprime']
      pdf_Zprime   = 1+xsecUnc['qq_PDF_Zprime']
      scale_Wprime = 1+xsecUnc['qq_scale_Wprime']
      scale_Zprime = 1+xsecUnc['qq_scale_Zprime']
      pdf_Bulk     = 1+xsecUnc['gg_PDF']
      scale_Bulk   = 1+xsecUnc['gg_scale']
      
      pdf   = pdf_Wprime
      scale = scale_Wprime
   
      if signal.find("Bulk") != -1:
        pdf   =  pdf_Bulk  
        scale =  scale_Bulk
      elif signal.find("Zprime") != -1:
          pdf   = pdf_Zprime
          scale = scale_Zprime
      # print "Mass = %f   Crossection = %f  ScaleUP =%f   ScaleDOWN = %f" %( float(split[0][-4:])/1000.,float(split[1]), (float(split[1])*scale    *pdf), float(split[1])*(1-(scale-1))*(1-(pdf-1)) )
      gtheory    .SetPoint(j, float(split[0][-4:])/1000., float(split[1]))
      gtheoryUP  .SetPoint(j, float(split[0][-4:])/1000., float(split[1])*scale    *pdf)
      gtheoryDOWN.SetPoint(j, float(split[0][-4:])/1000., float(split[1])*(1-(scale-1))*(1-(pdf-1)))
      radmasses.append(float(split[0][-4:])/1000.)
      ymin.append(float(split[1])*(1-(scale-1))*(1-(pdf-1)))
      ymax.append(float(split[1])*scale    *pdf)
      j+=1
  
  
  grshade = rt.TGraph(2*j)
  for i in range (0, j):
    grshade.SetPoint(i,radmasses[i],ymax[i])
    grshade.SetPoint(j+i,radmasses[j-i-1],ymin[j-i-1])

  grshade.SetFillStyle(3013);
  grshade.SetFillColor(rt.kRed);
  
  
  
  mg.Add(gtheory    )
  mg.Add(gtheoryUP  )
  mg.Add(gtheoryDOWN)
  # c1 = rt.TCanvas("c1","c1",50,50,W,H)
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
  c1.SetLogy()
  c1.SetGrid()
  c1.SetLogy()
  c1.cd()
  

  frame = c1.DrawFrame(1.1,0.001, 4.2, 10)
  frame.GetYaxis().CenterTitle()
  frame.GetYaxis().SetTitleSize(0.05)
  frame.GetXaxis().SetTitleSize(0.05)
  frame.GetXaxis().SetLabelSize(0.04)
  frame.GetYaxis().SetLabelSize(0.04)
  frame.GetYaxis().SetTitleOffset(1.15)
  frame.GetXaxis().SetTitleOffset(1.05)
  frame.GetXaxis().CenterTitle()
  frame.SetMinimum(0.0001)
  frame.SetMaximum(109.)
  
  mg    .Draw("Lsame")   
  grshade.Draw("fSAME"); 
  gtheoryUP  .Draw("Psame")
  gtheoryDOWN.Draw("Psame")
      
  # mg.Draw("APsame")
  
  print "Writing to file %sxSecUnc.root"%signal
  f = rt.TFile("%s_xSecUnc.root"%signal, "recreate")
  gtheory    .SetName("gtheory")
  gtheoryUP  .SetName("gtheoryUP")
  gtheoryDOWN.SetName("gtheoryDOWN")
  grshade    .SetName("grshade")

  gtheory    .Write()
  gtheoryUP  .Write()
  gtheoryDOWN.Write()
  grshade    .Write()
  f.Close()
  
  ftheory.close()
  del gtheory    
  del gtheoryUP  
  del gtheoryDOWN
  del grshade    