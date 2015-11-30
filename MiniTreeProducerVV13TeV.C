{


  double mgg, mjj,evWeight, mtot, normWeight;
 int categories;

 evWeight = 1.0;
 normWeight = 1.;


 string sInFile = "/shome/thaarres/EXOVVAnalysisRunII/LimitCode/CMSSW_7_1_5/src/DijetCombineLimitCode/input/Pseudodata.root";
 cout << sInFile.c_str() << endl;
 TFile file0(sInFile.c_str(), "read");
 

 string sOutFile("MiniTrees/Data_VV_13TeV/dijetVV_13TeV_miniTree.root");
 TFile f1(sOutFile.c_str(), "recreate");
 f1.cd();
 
 TTree *TCVARS = new TTree("TCVARS", "VV selection");
 TCVARS->Branch("mgg13TeV",&mgg,"mgg/D");
 
 TCVARS->Branch("evWeight",&evWeight,"evWeight/D");
 TCVARS->Branch("normWeight",&normWeight,"normWeight/D");
 
 TCVARS->Branch("categories",&categories,"categories/I");
 
 
 
 
 
 
 for (int iCat = 0; iCat < 21; iCat++){
   // TH1D* hMass = (TH1D*) file0.Get("DijetMassHighPuriVV"); // VV high purity
   // if (iCat == 1) hMass = (TH1D*) file0.Get("DijetMassMediumPuriVV"); // VV medium purity
   // if (iCat == 2) hMass = (TH1D*) file0.Get("DijetMassLowPuriVV"); // not used
   // if (iCat == 3) hMass = (TH1D*) file0.Get("DijetMassHighPuriqV"); // qV high purity
   // if (iCat == 4) hMass = (TH1D*) file0.Get("DijetMassMediumPuriqV"); // qV medium purity
   // if (iCat == 5) hMass = (TH1D*) file0.Get("DijetMassLowPuriqV"); // not used
   TH1D* hMass = (TH1D*) file0.Get("DijetMassHighPuriVV"); // WW high purity
   if (iCat == 1) hMass = (TH1D*) file0.Get("DijetMassLowPuriVV"); // WW low purity
   if (iCat == 2) hMass = (TH1D*) file0.Get("DijetMassNoPuriVV"); // WW no purity
   if (iCat == 3) hMass = (TH1D*) file0.Get("DijetMassHighPuriWW"); // WW high purity
   if (iCat == 4) hMass = (TH1D*) file0.Get("DijetMassLowPuriWW"); // WW low purity
   if (iCat == 5) hMass = (TH1D*) file0.Get("DijetMassNoPuriWW"); // WW no purity
   if (iCat == 6) hMass = (TH1D*) file0.Get("DijetMassHighPuriWZ"); // WZ high purity
   if (iCat == 7) hMass = (TH1D*) file0.Get("DijetMassLowPuriWZ"); // WZ low purity
   if (iCat == 8) hMass = (TH1D*) file0.Get("DijetMassNoPuriWZ"); // WZ no purity
   if (iCat == 9) hMass = (TH1D*) file0.Get("DijetMassHighPuriZZ"); // ZZ high purity
   if (iCat == 10) hMass = (TH1D*) file0.Get("DijetMassLowPuriZZ"); // ZZ low purity
   if (iCat == 11) hMass = (TH1D*) file0.Get("DijetMassNoPuriZZ"); // ZZ low purity
   if (iCat == 12) hMass = (TH1D*) file0.Get("DijetMassHighPuriqV"); // qZ high purity
   if (iCat == 13) hMass = (TH1D*) file0.Get("DijetMassLowPuriqV"); // qZ low purity
   if (iCat == 14) hMass = (TH1D*) file0.Get("DijetMassNoPuriqV"); // qZ low purity
   if (iCat == 15) hMass = (TH1D*) file0.Get("DijetMassHighPuriqW"); // qW high purity
   if (iCat == 16) hMass = (TH1D*) file0.Get("DijetMassLowPuriqW"); // qW low purity
   if (iCat == 17) hMass = (TH1D*) file0.Get("DijetMassNoPuriqW"); // qW low purity
   if (iCat == 18) hMass = (TH1D*) file0.Get("DijetMassHighPuriqZ"); // qZ high purity
   if (iCat == 19) hMass = (TH1D*) file0.Get("DijetMassLowPuriqZ"); // qZ low purity
   if (iCat == 20) hMass = (TH1D*) file0.Get("DijetMassNoPuriqZ"); // qZ low purity
   std::cout<<"Category = " << iCat << std::endl;
   std::cout<<"Histogram = " << hMass.GetName() << std::endl;
   TAxis* Axis =   hMass->GetXaxis();
   for (int i = 1 ; i < hMass->GetNbinsX()+1; i++){
     double N = abs(hMass->GetBinContent(i));
 
     if (i%10 == 0) cout << "i = " << i << "N = " << N << " binCenter = " << hMass->GetBinCenter(i) << endl;
     
     mgg = Axis->GetBinCenter(i);
     
     normWeight = N;
     categories = iCat;
     if (N > 1e-10) TCVARS->Fill();
     // if (N > 0) TCVARS->Fill();
   }
 }

 

 TCVARS->Write();
 f1.Close();
 file0.Close();
 

}


