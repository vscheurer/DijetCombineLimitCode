{


  double mgg, mjj,evWeight, mtot, normWeight;
 int categories;

 evWeight = 1.0;
 normWeight = 1.;

 //for (int iSample = 0; iSample < 7; iSample++){
   
   string inFile("BulkGtoZZ_bb_");
   //if (iSample == 1) inFile = string("RS1WW");
//    if (iSample == 2) inFile = string("RS1ZZ"); // Fake ZZ signal
//    if (iSample == 3) inFile = string("QstarQW");
//    if (iSample == 4) inFile = string("QstarQZ");
//    if (iSample == 5) inFile = string("BulkWW");
//    if (iSample == 6) inFile = string("BulkZZ");
//    if (iSample == 7) inFile = string("ZprimeWW");
//    if (iSample == 8) inFile = string("WprimeWZ");
// 
    string outFile("dijetVV_13TeV_ZZ");
//    if (iSample == 1) outFile = string("dijetVV_13TeV_RS1WW");
//    if (iSample == 2) outFile = string("dijetVV_13TeV_RS1ZZ");
//    if (iSample == 3) outFile = string("dijetVV_13TeV_QstarQW");
//    if (iSample == 4) outFile = string("dijetVV_13TeV_QstarQZ");
//    if (iSample == 5) outFile = string("dijetVV_13TeV_BulkWW");
//    if (iSample == 6) outFile = string("dijetVV_13TeV_BulkZZ");
//    if (iSample == 7) outFile = string("dijetVV_13TeV_ZprimeWW");
//    if (iSample == 8) outFile = string("dijetVV_13TeV_WZ");
//    
   int massrange=27;

   for (int iMass = 0; iMass<massrange; iMass++){

     string sInFile =inFile + Form("10k_OUT%dGeV.root", 1400+iMass*100);
     cout << sInFile.c_str() << endl;
     TFile file0(sInFile.c_str(), "read");

     string sOutFile = "MiniTrees/Signal_VV_13TeV/" + outFile + Form("OUT%d_miniTree.root", 1400+iMass*100);
     std::cout << "outfile name : "<< sOutFile << std::endl;
     TFile f1(sOutFile.c_str(), "recreate");
     f1.cd();

     TTree *TCVARS = new TTree("TCVARS", "VV selection");
     TCVARS->Branch("mgg13TeV",&mgg,"mgg/D");

     TCVARS->Branch("evWeight",&evWeight,"evWeight/D");
     TCVARS->Branch("normWeight",&normWeight,"normWeight/D");
     
     TCVARS->Branch("categories",&categories,"categories/I");

  
     double dMass = 1400.+iMass*100.;
     int intMass = 1400+iMass*100;
    std::stringstream sMass2;
    sMass2 << intMass;
    string sMass3 = sMass2.str();
    const char* sMass = sMass3.c_str(); 
    std::cout << sMass << std::endl;
    TH1D* hMass;
      
    
     for (int iCat = 0; iCat < 14; iCat++){
        string histname;
//         if (iCat == 0) {
//     histname = "BtaggedMass_"+sMass3+"GeV;1" ;  
//     std::cout << histname << std::endl;  
//         }   
 
   if (iCat == 0) histname = "VVHP_"+sMass3+"GeV;1" ;  
   if (iCat == 1) histname = "VVLP_"+sMass3+"GeV;1" ;
   if (iCat == 2) histname = "WWHP_"+sMass3+"GeV;1" ;  
   if (iCat == 3) histname = "WWLP_"+sMass3+"GeV;1" ;
   if (iCat == 4) histname = "WZHP_"+sMass3+"GeV;1" ;         
   if (iCat == 5) histname = "WZLP_"+sMass3+"GeV;1" ; 
                                                                                          
   if (iCat == 6) histname = "ZZHP_"+sMass3+"GeV;1" ;    
       std::cout << histname << std::endl;  

   if (iCat == 7) histname = "ZZLP_"+sMass3+"GeV;1" ;          
                                                                                          
      
                                                                                          

   
   if (iCat == 8) histname = "ZZHPBtagged_"+sMass3+"GeV;1" ;    
       std::cout << histname << std::endl;  

   if (iCat == 9) histname = "ZZLPBtagged_"+sMass3+"GeV;1" ;          
                                                                                          
   if (iCat == 10) histname = "WZHPBtagged_"+sMass3+"GeV;1" ;         
   if (iCat ==11) histname = "WZLPBtagged_"+sMass3+"GeV;1" ;       
                                                                                          
   if (iCat == 12) histname = "WWHPBtagged_"+sMass3+"GeV;1" ;  
   if (iCat == 13) histname = "WWLPBtagged_"+sMass3+"GeV;1" ;
                                                                                          
      /* 
       TH1D* hMass = (TH1D*) file0.Get("DijetMassHighPuriVV"); // WW high purity
       if (iCat == 1) hMass = (TH1D*) file0.Get("DijetMassLowPuriVV"); // WW low purity
       if (iCat == 2) hMass = (TH1D*) file0.Get("DijetMassHighPuriWW"); // WW high purity
       if (iCat == 3) hMass = (TH1D*) file0.Get("DijetMassLowPuriWW"); // WW low purity
       if (iCat == 4) hMass = (TH1D*) file0.Get("DijetMassHighPuriWZ"); // WZ high purity
       if (iCat == 5) hMass = (TH1D*) file0.Get("DijetMassLowPuriWZ"); // WZ low purity
       if (iCat == 6) hMass = (TH1D*) file0.Get("DijetMassHighPuriZZ"); // ZZ high purity
       if (iCat == 7) hMass = (TH1D*) file0.Get("DijetMassLowPuriZZ"); // ZZ low purity
       */
      TH1D* hMass = (TH1D*) file0.Get(histname.c_str());
      if (intMass == 1600 and iCat == 0 ){
      TCanvas* ctmp = new TCanvas(histname.c_str(),histname.c_str(),0,0,500,500);
      cout << hMass->GetEntries() << endl;}
       if(!hMass) {cout << "no histo!"<<endl;}
       else{
           cout << "histo" << endl;}
//        if (iCat == 2 and iMass == 14){
//        //TCanvas *c1 = new TCanvas(sMass, "",1345,52,640,480);
//          //  hMass->Draw("A");
//           test = hMass->Integral(1000.,5000.);
           cout << "histo " << histname << " cat " << iCat << endl;
        
       TAxis* Axis =   hMass->GetXaxis();
       for (int i = 1 ; i < hMass->GetNbinsX()+1; i++){
	 //if (hMass->GetBinCenter(i) < dMass*0.75 || hMass->GetBinCenter(i) > dMass*1.25) continue;
	 int N = abs(hMass->GetBinContent(i));
	 if (i%1000 == 0) cout << "i = " << i << " N = " << N << endl;
	 
	 mgg = Axis->GetBinCenter(i);
	 
         categories = iCat;
	 for (int k = 0; k < N; k++) {
           //std::cout << " cat " << categories << std::endl;
           //std::cout << " mgg " << mgg << std::endl;
           //std::cout << "evweight "<< evWeight << std::endl;
           //std::cout<< "normWeight" << normWeight << std::endl;
	   TCVARS->Fill();
           
	 }
       }
     }
     TCVARS->Print();
     TCVARS->Write();
                 
     f1.Close();
     file0.Close();
     
   }

 }






