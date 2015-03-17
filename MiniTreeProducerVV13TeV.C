{


  double mgg, mjj,evWeight, mtot, normWeight;
 int categories;

 evWeight = 1.0;
 normWeight = 1;


 string sInFile = "input/QCD_13TeV_1fb.root";
 cout << sInFile.c_str() << endl;
 TFile file0(sInFile.c_str(), "read");
 

 string sOutFile("MiniTrees/Data_VV_13TeV/dijetVV_13TeV_miniTree.root");
 TFile f1(sOutFile.c_str(), "recreate");
 f1.cd();
 
 TTree *TCVARS = new TTree("TCVARS", "VV selection");
 TCVARS->Branch("mgg",&mgg,"mgg/D");
 
 TCVARS->Branch("evWeight",&evWeight,"evWeight/D");
 TCVARS->Branch("normWeight",&normWeight,"normWeight/D");
 
 TCVARS->Branch("categories",&categories,"categories/I");
 
 
 
 
 
 
 for (int iCat = 0; iCat < 1; iCat++){
   TH1D* hMass = (TH1D*) file0.Get("DijetMassHighPuriVV"); // VV high purity
   if (iCat == 1) hMass = (TH1D*) file0.Get("DijetMassMediumPuriVV"); // VV medium purity
   if (iCat == 2) hMass = (TH1D*) file0.Get("DijetMassLowPuriVV"); // not used
   if (iCat == 3) hMass = (TH1D*) file0.Get("DijetMassHighPuriqV"); // qV high purity
   if (iCat == 4) hMass = (TH1D*) file0.Get("DijetMassMediumPuriqV"); // qV medium purity
   if (iCat == 5) hMass = (TH1D*) file0.Get("DijetMassLowPuriqV"); // not used
   

   TAxis* Axis =   hMass->GetXaxis();
   for (int i = 1 ; i < hMass->GetNbinsX()+1; i++){
     //////// TEMPORARILY ADD MISSING FACTOR 1000 from FB to PB
     int N = abs(hMass->GetBinContent(i)*1000.);
 
     if (i%10 == 0) cout << "i = " << i << "N = " << N << " binCenter = " << hMass->GetBinCenter(i) << endl;
     
     mgg = Axis->GetBinCenter(i);
     
     normWeight = N;
     categories = iCat;
     if (N > 1e-10) TCVARS->Fill();
   }
 }

 

 TCVARS->Write();
 f1.Close();
 file0.Close();
 

}


