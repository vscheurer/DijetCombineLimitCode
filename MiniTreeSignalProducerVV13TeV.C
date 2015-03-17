{


  double mgg, mjj,evWeight, mtot, normWeight;
 int categories;

 evWeight = 1.0;
 normWeight = 1;

 for (int iSample = 1; iSample < 3; iSample++){
   
   string inFile("Wprime");
   if (iSample == 1) inFile = string("RS1WW");
   //if (iSample == 2) inFile = string("RS1ZZ");
   if (iSample == 2) inFile = string("RS1WW"); // Fake ZZ signal
   if (iSample == 3) inFile = string("QstarQW");
   if (iSample == 4) inFile = string("QstarQZ");
   if (iSample == 5) inFile = string("BulkWW");
   if (iSample == 6) inFile = string("BulkZZ");

   string outFile("dijetVV_13TeV_WZ");
   if (iSample == 1) outFile = string("dijetVV_13TeV_RS1WW");
   if (iSample == 2) outFile = string("dijetVV_13TeV_RS1ZZ");
   if (iSample == 3) outFile = string("dijetVV_13TeV_QstarQW");
   if (iSample == 4) outFile = string("dijetVV_13TeV_QstarQZ");
   if (iSample == 5) outFile = string("dijetVV_13TeV_BulkWW");
   if (iSample == 6) outFile = string("dijetVV_13TeV_BulkZZ");
   
   int massrange=61;

   for (int iMass = 0; iMass<massrange; iMass++){

     string sInFile = "input/" + inFile + "_13TeV_" + Form("OUT%dGeV_1fb.root", 1000+iMass*50);
     cout << sInFile.c_str() << endl;
     TFile file0(sInFile.c_str(), "read");

     string sOutFile = "MiniTrees/Signal_VV_13TeV/" + outFile + Form("OUT%d_miniTree.root", 1000+iMass*50);
     TFile f1(sOutFile.c_str(), "recreate");
     f1.cd();

     TTree *TCVARS = new TTree("TCVARS", "VV selection");
     TCVARS->Branch("mgg",&mgg,"mgg/D");

     TCVARS->Branch("evWeight",&evWeight,"evWeight/D");
     TCVARS->Branch("normWeight",&normWeight,"normWeight/D");
     
     TCVARS->Branch("categories",&categories,"categories/I");

  
     double dMass = 1000.+iMass*50.;


     
     for (int iCat = 0; iCat < 1; iCat++){
      
       TH1D* hMass = (TH1D*) file0.Get("DijetMassHighPuriVV;1");
       if (iCat == 1) hMass = (TH1D*) file0.Get("DijetMassMediumPuriVV;1");
       if (iCat == 2) hMass = (TH1D*) file0.Get("DijetMassLowPuriVV;1");
       if (iCat == 3) hMass = (TH1D*) file0.Get("DijetMassHighPuriqV;1");
       if (iCat == 4) hMass = (TH1D*) file0.Get("DijetMassMediumPuriqV;1");
       if (iCat == 5) hMass = (TH1D*) file0.Get("DijetMassLowPuriqV;1");
       if(!hMass) continue;
       
       TAxis* Axis =   hMass->GetXaxis();
       for (int i = 1 ; i < hMass->GetNbinsX()+1; i++){
	 //if (hMass->GetBinCenter(i) < dMass*0.75 || hMass->GetBinCenter(i) > dMass*1.25) continue;
	 int N = abs(hMass->GetBinContent(i));
	 if (i%1000 == 0) cout << "i = " << i << " N = " << N << endl;
	 
	 mgg = Axis->GetBinCenter(i);
	 
         categories = iCat;
	 for (int k = 0; k < N; k++) {
	   TCVARS->Fill();
	 }
       }
     }
   

     TCVARS->Write();
     f1.Close();
     file0.Close();
     
   }

 }


}


