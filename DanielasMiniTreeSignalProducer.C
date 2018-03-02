void DanielasMiniTreeSignalProducer(int samplemin, int samplemax){

 string dir = "/usr/users/vscheurer/DijetCombineLimitCode/";
  double mgg, mjj,evWeight, mtot, normWeight;
 int categories;

 evWeight = 1.0;
 normWeight = 1;

 for (int iSample = samplemin; iSample < samplemax; iSample++){
   
   string inFile("BulkGtoZZ_bb_");
   if (iSample == 1) inFile = string("RS1WW");
   if (iSample == 2) inFile = string("RS1ZZ"); // Fake ZZ signal
   if (iSample == 3) inFile = string("QstarQW");
   if (iSample == 4) inFile = string("QstarQZ");
   if (iSample == 5) inFile = string("BulkWW");
   if (iSample == 6) inFile = string("BulkZZ");
   if (iSample == 7) inFile = string("ZprimeWW");
   if (iSample == 8) inFile = string("WprimeWZ");

   string outFile("dijetVV_13TeV_WZ");
   if (iSample == 1) outFile = string("dijetVV_13TeV_RS1WW");
   if (iSample == 2) outFile = string("dijetVV_13TeV_RS1ZZ");
   if (iSample == 3) outFile = string("dijetVV_13TeV_QstarQW");
   if (iSample == 4) outFile = string("dijetVV_13TeV_QstarQZ");
   if (iSample == 5) outFile = string("dijetVV_13TeV_BulkWW");
   if (iSample == 6) outFile = string("dijetVV_13TeV_BulkZZ");
   if (iSample == 7) outFile = string("dijetVV_13TeV_ZprimeWW");
   if (iSample == 8) outFile = string("dijetVV_13TeV_WZ");
   
   int massrange=27; //36;

   for (int iMass = 0; iMass<massrange; iMass++){

     string sInFile = dir + inFile  + Form("10k_OUT%dGeV.root", 1400+iMass*100);
     cout << sInFile.c_str() << endl;
     TFile file0(sInFile.c_str(), "read");

     string sOutFile = dir+"MiniTrees/Signal_VV_13TeV/" + outFile + Form("OUT%dtest_miniTree.root", 1400+iMass*100);
     TFile f1(sOutFile.c_str(), "recreate");
     f1.cd();

     TTree *TCVARS = new TTree("TCVARS", "VV selection");
     TCVARS->Branch("mgg13TeV",&mgg,"mgg/D");

     TCVARS->Branch("evWeight",&evWeight,"evWeight/D");
     TCVARS->Branch("normWeight",&normWeight,"normWeight/D");
     
     TCVARS->Branch("categories",&categories,"categories/I");

  
     double dMass = 1400.+iMass*100.;

     
     for (int iCat = 2; iCat < 8; iCat++){
       TH1D* hMass = (TH1D*) file0.Get("VV;1");
       if (iCat == 1) hMass = (TH1D*) file0.Get("VV;1");
       
       if (iCat == 2) hMass = (TH1D*) file0.Get("WWHP;1");
       if (iCat == 3) hMass = (TH1D*) file0.Get("WWLP;1");
       
       if (iCat == 4) hMass = (TH1D*) file0.Get("WZHP;1");
       if (iCat == 5) hMass = (TH1D*) file0.Get("WZLP;1");
       
       if (iCat == 6) hMass = (TH1D*) file0.Get("ZZHP;1");
       if (iCat == 7) hMass = (TH1D*) file0.Get("ZZLP;1");
    
       if (iCat == 8) hMass = (TH1D*) file0.Get("qV;1");
       if (iCat == 9) hMass = (TH1D*) file0.Get("qV;1");

       if (iCat == 10) hMass = (TH1D*) file0.Get("qW;1");
       if (iCat == 11) hMass = (TH1D*) file0.Get("qW;1");

       if (iCat == 12) hMass = (TH1D*) file0.Get("qZ;1");
       if (iCat == 13) hMass = (TH1D*) file0.Get("qZ;1");

       if (iCat == 14) hMass = (TH1D*) file0.Get("DijetMassNoPuriVV;1");
       if (iCat == 15) hMass = (TH1D*) file0.Get("DijetMassNoPuriWW;1");
       if (iCat == 16) hMass = (TH1D*) file0.Get("DijetMassNoPuriWZ;1");
       if (iCat == 17) hMass = (TH1D*) file0.Get("DijetMassNoPuriZZ;1");
       if (iCat == 18) hMass = (TH1D*) file0.Get("DijetMassNoPuriqV;1");
       if (iCat == 19) hMass = (TH1D*) file0.Get("DijetMassNoPuriqW;1");
       if (iCat == 20) hMass = (TH1D*) file0.Get("DijetMassNoPuriqZ;1");
       // TH1D* hMass = (TH1D*) file0.Get("VV"); // WW high purity
//        if (iCat == 1) hMass = (TH1D*) file0.Get("VV"); // WW low purity
//        if (iCat == 2) hMass = (TH1D*) file0.Get("WW"); // WW high purity
//        if (iCat == 3) hMass = (TH1D*) file0.Get("WW"); // WW low purity
//        if (iCat == 4) hMass = (TH1D*) file0.Get("WZ"); // WZ high purity
//        if (iCat == 5) hMass = (TH1D*) file0.Get("WZ"); // WZ low purity
//        if (iCat == 6) hMass = (TH1D*) file0.Get("ZZ"); // ZZ high purity
//        if (iCat == 7) hMass = (TH1D*) file0.Get("ZZ"); // ZZ low purity
       
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


