void brazilianFlag_valerieBulkZZ_VVnew_new_combined_13TeV()
{
//=========Macro generated from canvas: c1/c1
//=========  (Thu Aug 17 10:46:41 2017) by ROOT version6.02/05
   TCanvas *c1 = new TCanvas("c1", "c1",66,78,800,600);
   gStyle->SetOptFit(1);
   gStyle->SetOptStat(0);
   gStyle->SetOptTitle(0);
   c1->SetHighLightColor(2);
   c1->Range(0.7857143,-4.905614,4.238095,2.641169);
   c1->SetFillColor(0);
   c1->SetBorderMode(0);
   c1->SetBorderSize(2);
   c1->SetLogy();
   c1->SetLeftMargin(0.12);
   c1->SetRightMargin(0.04);
   c1->SetTopMargin(0.08);
   c1->SetBottomMargin(0.12);
   c1->SetFrameFillStyle(0);
   c1->SetFrameBorderMode(0);
   c1->SetFrameFillStyle(0);
   c1->SetFrameBorderMode(0);
   
   TH1F *hframe1 = new TH1F("hframe1","",1000,1.2,4.1);
   hframe1->SetMinimum(0.0001);
   hframe1->SetMaximum(109);
   hframe1->SetDirectory(0);
   hframe1->SetStats(0);
   hframe1->SetLineStyle(0);
   hframe1->SetMarkerStyle(20);
   hframe1->GetXaxis()->SetTitle("M_{G_{Bulk}} (TeV)");
   hframe1->GetXaxis()->SetNdivisions(508);
   hframe1->GetXaxis()->SetLabelFont(42);
   hframe1->GetXaxis()->SetLabelOffset(0.007);
   hframe1->GetXaxis()->SetTitleSize(0.05);
   hframe1->GetXaxis()->SetTitleOffset(1.05);
   hframe1->GetXaxis()->SetTitleFont(42);
   hframe1->GetYaxis()->SetTitle("#sigma #times #bf{#it{#Beta}}(G_{Bulk} #rightarrow valerieZZ) (pb)");
   hframe1->GetYaxis()->SetLabelFont(42);
   hframe1->GetYaxis()->SetLabelOffset(0.007);
   hframe1->GetYaxis()->SetTitleSize(0.05);
   hframe1->GetYaxis()->SetTitleOffset(1.15);
   hframe1->GetYaxis()->SetTitleFont(42);
   hframe1->GetZaxis()->SetLabelFont(42);
   hframe1->GetZaxis()->SetLabelOffset(0.007);
   hframe1->GetZaxis()->SetLabelSize(0.05);
   hframe1->GetZaxis()->SetTitleSize(0.06);
   hframe1->GetZaxis()->SetTitleFont(42);
   hframe1->Draw(" ");
   
   Double_t Graph0_fx1[58] = {
   1.2,
   1.3,
   1.4,
   1.5,
   1.6,
   1.7,
   1.8,
   1.9,
   2,
   2.1,
   2.2,
   2.3,
   2.4,
   2.5,
   2.6,
   2.7,
   2.8,
   2.9,
   3,
   3.1,
   3.2,
   3.3,
   3.4,
   3.5,
   3.6,
   3.7,
   3.8,
   3.9,
   4,
   4,
   3.9,
   3.8,
   3.7,
   3.6,
   3.5,
   3.4,
   3.3,
   3.2,
   3.1,
   3,
   2.9,
   2.8,
   2.7,
   2.6,
   2.5,
   2.4,
   2.3,
   2.2,
   2.1,
   2,
   1.9,
   1.8,
   1.7,
   1.6,
   1.5,
   1.4,
   1.3,
   1.2};
   Double_t Graph0_fy1[58] = {
   0.001404708,
   0.001610618,
   0.008390053,
   0.006472326,
   0.005055301,
   0.003999019,
   0.003248749,
   0.002636386,
   0.002153905,
   0.001831162,
   0.001569543,
   0.001378658,
   0.001223824,
   0.00109921,
   0.000995401,
   0.000906593,
   0.0008215168,
   0.0007438068,
   0.0006863901,
   0.0006223391,
   0.000581362,
   0.000540141,
   0.0005000665,
   0.0004702117,
   0.000440357,
   0.0004039655,
   0.0003848429,
   0.0003609396,
   0.0003513783,
   0.002782411,
   0.00285824,
   0.003047403,
   0.003198563,
   0.003350112,
   0.003576944,
   0.003804052,
   0.004068844,
   0.004295942,
   0.004598551,
   0.004910346,
   0.005283413,
   0.00564902,
   0.006142529,
   0.006629833,
   0.007141922,
   0.00778624,
   0.008627818,
   0.009650064,
   0.01097051,
   0.01269013,
   0.01518568,
   0.01829038,
   0.02213497,
   0.02721024,
   0.03424322,
   0.04349791,
   0.007507132,
   0.007207293};
   TGraph *graph = new TGraph(58,Graph0_fx1,Graph0_fy1);
   graph->SetName("Graph0");
   graph->SetTitle("Graph");

   Int_t ci;      // for color index setting
   TColor *color; // for color definition with alpha
   ci = TColor::GetColor("#ffcc00");
   graph->SetFillColor(ci);

   ci = TColor::GetColor("#ffcc00");
   graph->SetLineColor(ci);
   graph->SetMarkerStyle(20);
   
   TH1F *Graph_Graph1 = new TH1F("Graph_Graph1","Graph",100,0.92,4.28);
   Graph_Graph1->SetMinimum(0.0003162404);
   Graph_Graph1->SetMaximum(0.04781256);
   Graph_Graph1->SetDirectory(0);
   Graph_Graph1->SetStats(0);
   Graph_Graph1->SetLineStyle(0);
   Graph_Graph1->SetMarkerStyle(20);
   Graph_Graph1->GetXaxis()->SetLabelFont(42);
   Graph_Graph1->GetXaxis()->SetLabelOffset(0.007);
   Graph_Graph1->GetXaxis()->SetLabelSize(0.05);
   Graph_Graph1->GetXaxis()->SetTitleSize(0.06);
   Graph_Graph1->GetXaxis()->SetTitleOffset(0.9);
   Graph_Graph1->GetXaxis()->SetTitleFont(42);
   Graph_Graph1->GetYaxis()->SetLabelFont(42);
   Graph_Graph1->GetYaxis()->SetLabelOffset(0.007);
   Graph_Graph1->GetYaxis()->SetLabelSize(0.05);
   Graph_Graph1->GetYaxis()->SetTitleSize(0.06);
   Graph_Graph1->GetYaxis()->SetTitleOffset(1.25);
   Graph_Graph1->GetYaxis()->SetTitleFont(42);
   Graph_Graph1->GetZaxis()->SetLabelFont(42);
   Graph_Graph1->GetZaxis()->SetLabelOffset(0.007);
   Graph_Graph1->GetZaxis()->SetLabelSize(0.05);
   Graph_Graph1->GetZaxis()->SetTitleSize(0.06);
   Graph_Graph1->GetZaxis()->SetTitleFont(42);
   graph->SetHistogram(Graph_Graph1);
   
   graph->Draw("f");
   
   Double_t Graph1_fx2[58] = {
   1.2,
   1.3,
   1.4,
   1.5,
   1.6,
   1.7,
   1.8,
   1.9,
   2,
   2.1,
   2.2,
   2.3,
   2.4,
   2.5,
   2.6,
   2.7,
   2.8,
   2.9,
   3,
   3.1,
   3.2,
   3.3,
   3.4,
   3.5,
   3.6,
   3.7,
   3.8,
   3.9,
   4,
   4,
   3.9,
   3.8,
   3.7,
   3.6,
   3.5,
   3.4,
   3.3,
   3.2,
   3.1,
   3,
   2.9,
   2.8,
   2.7,
   2.6,
   2.5,
   2.4,
   2.3,
   2.2,
   2.1,
   2,
   1.9,
   1.8,
   1.7,
   1.6,
   1.5,
   1.4,
   1.3,
   1.2};
   Double_t Graph1_fy2[58] = {
   0.001957464,
   0.002209601,
   0.01171169,
   0.009019202,
   0.007075995,
   0.005622771,
   0.00455929,
   0.003732378,
   0.003070709,
   0.002605573,
   0.002251453,
   0.001980422,
   0.001766534,
   0.001587193,
   0.001444392,
   0.001317328,
   0.001193708,
   0.001107944,
   0.001007484,
   0.0009229974,
   0.0008622239,
   0.0008096789,
   0.0007537,
   0.000708703,
   0.000663706,
   0.0006176705,
   0.0005884316,
   0.0005518831,
   0.0005372637,
   0.001671585,
   0.001732104,
   0.001830784,
   0.001888104,
   0.001995103,
   0.002092732,
   0.002225604,
   0.002359216,
   0.002490893,
   0.002642271,
   0.002859741,
   0.003070585,
   0.003329161,
   0.003639979,
   0.003923276,
   0.004252977,
   0.004671989,
   0.005216705,
   0.005831174,
   0.006679528,
   0.007789357,
   0.009321801,
   0.01132664,
   0.01382576,
   0.01716134,
   0.02179091,
   0.02766744,
   0.0049874,
   0.004670359};
   graph = new TGraph(58,Graph1_fx2,Graph1_fy2);
   graph->SetName("Graph1");
   graph->SetTitle("Graph");

   ci = TColor::GetColor("#00cc00");
   graph->SetFillColor(ci);

   ci = TColor::GetColor("#00cc00");
   graph->SetLineColor(ci);
   graph->SetMarkerStyle(20);
   
   TH1F *Graph_Graph2 = new TH1F("Graph_Graph2","Graph",100,0.92,4.28);
   Graph_Graph2->SetMinimum(0.0004835373);
   Graph_Graph2->SetMaximum(0.03038046);
   Graph_Graph2->SetDirectory(0);
   Graph_Graph2->SetStats(0);
   Graph_Graph2->SetLineStyle(0);
   Graph_Graph2->SetMarkerStyle(20);
   Graph_Graph2->GetXaxis()->SetLabelFont(42);
   Graph_Graph2->GetXaxis()->SetLabelOffset(0.007);
   Graph_Graph2->GetXaxis()->SetLabelSize(0.05);
   Graph_Graph2->GetXaxis()->SetTitleSize(0.06);
   Graph_Graph2->GetXaxis()->SetTitleOffset(0.9);
   Graph_Graph2->GetXaxis()->SetTitleFont(42);
   Graph_Graph2->GetYaxis()->SetLabelFont(42);
   Graph_Graph2->GetYaxis()->SetLabelOffset(0.007);
   Graph_Graph2->GetYaxis()->SetLabelSize(0.05);
   Graph_Graph2->GetYaxis()->SetTitleSize(0.06);
   Graph_Graph2->GetYaxis()->SetTitleOffset(1.25);
   Graph_Graph2->GetYaxis()->SetTitleFont(42);
   Graph_Graph2->GetZaxis()->SetLabelFont(42);
   Graph_Graph2->GetZaxis()->SetLabelOffset(0.007);
   Graph_Graph2->GetZaxis()->SetLabelSize(0.05);
   Graph_Graph2->GetZaxis()->SetTitleSize(0.06);
   Graph_Graph2->GetZaxis()->SetTitleFont(42);
   graph->SetHistogram(Graph_Graph2);
   
   graph->Draw("f");
   
   Double_t Graph2_fx1001[29] = {
   1.2,
   1.3,
   1.4,
   1.5,
   1.6,
   1.7,
   1.8,
   1.9,
   2,
   2.1,
   2.2,
   2.3,
   2.4,
   2.5,
   2.6,
   2.7,
   2.8,
   2.9,
   3,
   3.1,
   3.2,
   3.3,
   3.4,
   3.5,
   3.6,
   3.7,
   3.8,
   3.9,
   4};
   Double_t Graph2_fy1001[29] = {
   0.002959714,
   0.003259432,
   0.0175335,
   0.01363716,
   0.01073989,
   0.008566937,
   0.007018394,
   0.005719617,
   0.004733045,
   0.00405868,
   0.003509197,
   0.003109573,
   0.002784879,
   0.002535114,
   0.00231657,
   0.002129246,
   0.001929434,
   0.001779575,
   0.001642204,
   0.001517322,
   0.001417416,
   0.001342486,
   0.001255069,
   0.001180139,
   0.00110521,
   0.001055257,
   0.001005304,
   0.0009428626,
   0.0009178861};
   Double_t Graph2_fex1001[29] = {
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0};
   Double_t Graph2_fey1001[29] = {
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0};
   gre = new TGraphErrors(29,Graph2_fx1001,Graph2_fy1001,Graph2_fex1001,Graph2_fey1001);
   gre->SetName("Graph2");
   gre->SetTitle("Graph");
   gre->SetFillColor(1);
   gre->SetLineStyle(3);
   gre->SetLineWidth(4);
   gre->SetMarkerStyle(20);
   
   TH1F *Graph_Graph1001 = new TH1F("Graph_Graph1001","Graph",100,0.92,4.28);
   Graph_Graph1001->SetMinimum(0.0008260975);
   Graph_Graph1001->SetMaximum(0.01919506);
   Graph_Graph1001->SetDirectory(0);
   Graph_Graph1001->SetStats(0);
   Graph_Graph1001->SetLineStyle(0);
   Graph_Graph1001->SetMarkerStyle(20);
   Graph_Graph1001->GetXaxis()->SetLabelFont(42);
   Graph_Graph1001->GetXaxis()->SetLabelOffset(0.007);
   Graph_Graph1001->GetXaxis()->SetLabelSize(0.05);
   Graph_Graph1001->GetXaxis()->SetTitleSize(0.06);
   Graph_Graph1001->GetXaxis()->SetTitleOffset(0.9);
   Graph_Graph1001->GetXaxis()->SetTitleFont(42);
   Graph_Graph1001->GetYaxis()->SetLabelFont(42);
   Graph_Graph1001->GetYaxis()->SetLabelOffset(0.007);
   Graph_Graph1001->GetYaxis()->SetLabelSize(0.05);
   Graph_Graph1001->GetYaxis()->SetTitleSize(0.06);
   Graph_Graph1001->GetYaxis()->SetTitleOffset(1.25);
   Graph_Graph1001->GetYaxis()->SetTitleFont(42);
   Graph_Graph1001->GetZaxis()->SetLabelFont(42);
   Graph_Graph1001->GetZaxis()->SetLabelOffset(0.007);
   Graph_Graph1001->GetZaxis()->SetLabelSize(0.05);
   Graph_Graph1001->GetZaxis()->SetTitleSize(0.06);
   Graph_Graph1001->GetZaxis()->SetTitleFont(42);
   gre->SetHistogram(Graph_Graph1001);
   
   gre->Draw("l");
   
   Double_t Graph3_fx1002[29] = {
   1.2,
   1.3,
   1.4,
   1.5,
   1.6,
   1.7,
   1.8,
   1.9,
   2,
   2.1,
   2.2,
   2.3,
   2.4,
   2.5,
   2.6,
   2.7,
   2.8,
   2.9,
   3,
   3.1,
   3.2,
   3.3,
   3.4,
   3.5,
   3.6,
   3.7,
   3.8,
   3.9,
   4};
   Double_t Graph3_fy1002[29] = {
   0.002426217,
   0.002138344,
   0.0228992,
   0.01516699,
   0.005905449,
   0.008066655,
   0.008491373,
   0.00659328,
   0.005840914,
   0.005230486,
   0.005639083,
   0.005016342,
   0.003363597,
   0.002295413,
   0.002347059,
   0.002692798,
   0.002665596,
   0.002666833,
   0.002742446,
   0.002532582,
   0.002349563,
   0.002073908,
   0.001202512,
   0.0007265559,
   0.0006026756,
   0.0005760372,
   0.0005883233,
   0.0005952333,
   0.0006078811};
   Double_t Graph3_fex1002[29] = {
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0};
   Double_t Graph3_fey1002[29] = {
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0};
   gre = new TGraphErrors(29,Graph3_fx1002,Graph3_fy1002,Graph3_fex1002,Graph3_fey1002);
   gre->SetName("Graph3");
   gre->SetTitle("Graph");
   gre->SetFillColor(1);
   gre->SetLineWidth(2);
   gre->SetMarkerStyle(8);
   gre->SetMarkerSize(0.8);
   
   TH1F *Graph_Graph1002 = new TH1F("Graph_Graph1002","Graph",100,0.92,4.28);
   Graph_Graph1002->SetMinimum(0.0005184335);
   Graph_Graph1002->SetMaximum(0.02513152);
   Graph_Graph1002->SetDirectory(0);
   Graph_Graph1002->SetStats(0);
   Graph_Graph1002->SetLineStyle(0);
   Graph_Graph1002->SetMarkerStyle(20);
   Graph_Graph1002->GetXaxis()->SetLabelFont(42);
   Graph_Graph1002->GetXaxis()->SetLabelOffset(0.007);
   Graph_Graph1002->GetXaxis()->SetLabelSize(0.05);
   Graph_Graph1002->GetXaxis()->SetTitleSize(0.06);
   Graph_Graph1002->GetXaxis()->SetTitleOffset(0.9);
   Graph_Graph1002->GetXaxis()->SetTitleFont(42);
   Graph_Graph1002->GetYaxis()->SetLabelFont(42);
   Graph_Graph1002->GetYaxis()->SetLabelOffset(0.007);
   Graph_Graph1002->GetYaxis()->SetLabelSize(0.05);
   Graph_Graph1002->GetYaxis()->SetTitleSize(0.06);
   Graph_Graph1002->GetYaxis()->SetTitleOffset(1.25);
   Graph_Graph1002->GetYaxis()->SetTitleFont(42);
   Graph_Graph1002->GetZaxis()->SetLabelFont(42);
   Graph_Graph1002->GetZaxis()->SetLabelOffset(0.007);
   Graph_Graph1002->GetZaxis()->SetLabelSize(0.05);
   Graph_Graph1002->GetZaxis()->SetTitleSize(0.06);
   Graph_Graph1002->GetZaxis()->SetTitleFont(42);
   gre->SetHistogram(Graph_Graph1002);
   
   gre->Draw("lp");
   
   Double_t valerieBulkZZ_gtheory_fx1003[11] = {
   1,
   1.2,
   1.4,
   1.6,
   1.8,
   2,
   2.5,
   3,
   3.5,
   4,
   4.5};
   Double_t valerieBulkZZ_gtheory_fy1003[11] = {
   0.01024985,
   0.00341978,
   0.00130687,
   0.0005741872,
   0.0002441465,
   0.0001197594,
   2.242854e-05,
   4.91243e-06,
   2.098791e-06,
   1.219122e-06,
   1.044836e-06};
   Double_t valerieBulkZZ_gtheory_fex1003[11] = {
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0};
   Double_t valerieBulkZZ_gtheory_fey1003[11] = {
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0,
   0};
   gre = new TGraphErrors(11,valerieBulkZZ_gtheory_fx1003,valerieBulkZZ_gtheory_fy1003,valerieBulkZZ_gtheory_fex1003,valerieBulkZZ_gtheory_fey1003);
   gre->SetName("valerieBulkZZ_gtheory");
   gre->SetTitle("Graph");
   gre->SetFillColor(1);

   ci = TColor::GetColor("#ff0000");
   gre->SetLineColor(ci);
   gre->SetLineWidth(3);
   
   TH1F *Graph_valerieBulkZZ_gtheory1003 = new TH1F("Graph_valerieBulkZZ_gtheory1003","Graph",100,0.65,4.85);
   Graph_valerieBulkZZ_gtheory1003->SetMinimum(9.403527e-07);
   Graph_valerieBulkZZ_gtheory1003->SetMaximum(0.01127473);
   Graph_valerieBulkZZ_gtheory1003->SetDirectory(0);
   Graph_valerieBulkZZ_gtheory1003->SetStats(0);
   Graph_valerieBulkZZ_gtheory1003->SetLineStyle(0);
   Graph_valerieBulkZZ_gtheory1003->SetMarkerStyle(20);
   Graph_valerieBulkZZ_gtheory1003->GetXaxis()->SetLabelFont(42);
   Graph_valerieBulkZZ_gtheory1003->GetXaxis()->SetLabelOffset(0.007);
   Graph_valerieBulkZZ_gtheory1003->GetXaxis()->SetLabelSize(0.05);
   Graph_valerieBulkZZ_gtheory1003->GetXaxis()->SetTitleSize(0.06);
   Graph_valerieBulkZZ_gtheory1003->GetXaxis()->SetTitleOffset(0.9);
   Graph_valerieBulkZZ_gtheory1003->GetXaxis()->SetTitleFont(42);
   Graph_valerieBulkZZ_gtheory1003->GetYaxis()->SetLabelFont(42);
   Graph_valerieBulkZZ_gtheory1003->GetYaxis()->SetLabelOffset(0.007);
   Graph_valerieBulkZZ_gtheory1003->GetYaxis()->SetLabelSize(0.05);
   Graph_valerieBulkZZ_gtheory1003->GetYaxis()->SetTitleSize(0.06);
   Graph_valerieBulkZZ_gtheory1003->GetYaxis()->SetTitleOffset(1.25);
   Graph_valerieBulkZZ_gtheory1003->GetYaxis()->SetTitleFont(42);
   Graph_valerieBulkZZ_gtheory1003->GetZaxis()->SetLabelFont(42);
   Graph_valerieBulkZZ_gtheory1003->GetZaxis()->SetLabelOffset(0.007);
   Graph_valerieBulkZZ_gtheory1003->GetZaxis()->SetLabelSize(0.05);
   Graph_valerieBulkZZ_gtheory1003->GetZaxis()->SetTitleSize(0.06);
   Graph_valerieBulkZZ_gtheory1003->GetZaxis()->SetTitleFont(42);
   gre->SetHistogram(Graph_valerieBulkZZ_gtheory1003);
   
   gre->Draw("l");
   
   Double_t valerieBulkZZ_grshade_fx3[22] = {
   1,
   1.2,
   1.4,
   1.6,
   1.8,
   2,
   2.5,
   3,
   3.5,
   4,
   4.5,
   4.5,
   4,
   3.5,
   3,
   2.5,
   2,
   1.8,
   1.6,
   1.4,
   1.2,
   1};
   Double_t valerieBulkZZ_grshade_fy3[22] = {
   0.0134007,
   0.004611356,
   0.001811763,
   0.0008217326,
   0.0003600711,
   0.0001818294,
   3.647573e-05,
   8.680007e-06,
   4.007235e-06,
   2.568563e-06,
   2.408994e-06,
   9.808328e-08,
   2.553502e-07,
   6.904563e-07,
   2.048e-06,
   1.139506e-05,
   6.948064e-05,
   0.000148981,
   0.0003680936,
   0.0008800111,
   0.002397691,
   0.007501346};
   graph = new TGraph(22,valerieBulkZZ_grshade_fx3,valerieBulkZZ_grshade_fy3);
   graph->SetName("valerieBulkZZ_grshade");
   graph->SetTitle("Graph");

   ci = TColor::GetColor("#ff0000");
   graph->SetFillColor(ci);
   graph->SetFillStyle(3013);
   graph->SetLineColor(0);
   
   TH1F *Graph_valerieBulkZZ_grshade3 = new TH1F("Graph_valerieBulkZZ_grshade3","Graph",100,0.65,4.85);
   Graph_valerieBulkZZ_grshade3->SetMinimum(8.827495e-08);
   Graph_valerieBulkZZ_grshade3->SetMaximum(0.01474076);
   Graph_valerieBulkZZ_grshade3->SetDirectory(0);
   Graph_valerieBulkZZ_grshade3->SetStats(0);
   Graph_valerieBulkZZ_grshade3->SetLineStyle(0);
   Graph_valerieBulkZZ_grshade3->SetMarkerStyle(20);
   Graph_valerieBulkZZ_grshade3->GetXaxis()->SetLabelFont(42);
   Graph_valerieBulkZZ_grshade3->GetXaxis()->SetLabelOffset(0.007);
   Graph_valerieBulkZZ_grshade3->GetXaxis()->SetLabelSize(0.05);
   Graph_valerieBulkZZ_grshade3->GetXaxis()->SetTitleSize(0.06);
   Graph_valerieBulkZZ_grshade3->GetXaxis()->SetTitleOffset(0.9);
   Graph_valerieBulkZZ_grshade3->GetXaxis()->SetTitleFont(42);
   Graph_valerieBulkZZ_grshade3->GetYaxis()->SetLabelFont(42);
   Graph_valerieBulkZZ_grshade3->GetYaxis()->SetLabelOffset(0.007);
   Graph_valerieBulkZZ_grshade3->GetYaxis()->SetLabelSize(0.05);
   Graph_valerieBulkZZ_grshade3->GetYaxis()->SetTitleSize(0.06);
   Graph_valerieBulkZZ_grshade3->GetYaxis()->SetTitleOffset(1.25);
   Graph_valerieBulkZZ_grshade3->GetYaxis()->SetTitleFont(42);
   Graph_valerieBulkZZ_grshade3->GetZaxis()->SetLabelFont(42);
   Graph_valerieBulkZZ_grshade3->GetZaxis()->SetLabelOffset(0.007);
   Graph_valerieBulkZZ_grshade3->GetZaxis()->SetLabelSize(0.05);
   Graph_valerieBulkZZ_grshade3->GetZaxis()->SetTitleSize(0.06);
   Graph_valerieBulkZZ_grshade3->GetZaxis()->SetTitleFont(42);
   graph->SetHistogram(Graph_valerieBulkZZ_grshade3);
   
   graph->Draw("f");
   
   TPaveText *pt = new TPaveText(0.52,0.2,0.8,0.9,"brNDC");
   pt->SetBorderSize(0);
   pt->SetFillColor(0);
   pt->SetFillStyle(0);
   pt->SetLineColor(0);
   pt->SetTextAlign(12);
   pt->SetTextFont(42);
   pt->SetTextSize(0.035);
   TText *AText = pt->AddText("Narrow width approximation");
   pt->Draw();
      tex = new TLatex(0.96,0.936,"35.9 fb^{-1} (13 TeV)");
tex->SetNDC();
   tex->SetTextAlign(31);
   tex->SetTextFont(42);
   tex->SetTextSize(0.048);
   tex->SetLineWidth(2);
   tex->Draw();
      tex = new TLatex(0.1578,0.892,"CMS");
tex->SetNDC();
   tex->SetTextAlign(13);
   tex->SetTextFont(61);
   tex->SetTextSize(0.06);
   tex->SetLineWidth(2);
   tex->Draw();
      tex = new TLatex(0.1578,0.82,"");
tex->SetNDC();
   tex->SetTextAlign(13);
   tex->SetTextFont(52);
   tex->SetTextSize(0.0456);
   tex->SetLineWidth(2);
   tex->Draw();
   
   TH1F *hframe_copy2 = new TH1F("hframe_copy2","",1000,1.2,4.1);
   hframe_copy2->SetMinimum(0.0001);
   hframe_copy2->SetMaximum(109);
   hframe_copy2->SetDirectory(0);
   hframe_copy2->SetStats(0);
   hframe_copy2->SetLineStyle(0);
   hframe_copy2->SetMarkerStyle(20);
   hframe_copy2->GetXaxis()->SetTitle("M_{G_{Bulk}} (TeV)");
   hframe_copy2->GetXaxis()->SetNdivisions(508);
   hframe_copy2->GetXaxis()->SetLabelFont(42);
   hframe_copy2->GetXaxis()->SetLabelOffset(0.007);
   hframe_copy2->GetXaxis()->SetTitleSize(0.05);
   hframe_copy2->GetXaxis()->SetTitleOffset(1.05);
   hframe_copy2->GetXaxis()->SetTitleFont(42);
   hframe_copy2->GetYaxis()->SetTitle("#sigma #times #bf{#it{#Beta}}(G_{Bulk} #rightarrow valerieZZ) (pb)");
   hframe_copy2->GetYaxis()->SetLabelFont(42);
   hframe_copy2->GetYaxis()->SetLabelOffset(0.007);
   hframe_copy2->GetYaxis()->SetTitleSize(0.05);
   hframe_copy2->GetYaxis()->SetTitleOffset(1.15);
   hframe_copy2->GetYaxis()->SetTitleFont(42);
   hframe_copy2->GetZaxis()->SetLabelFont(42);
   hframe_copy2->GetZaxis()->SetLabelOffset(0.007);
   hframe_copy2->GetZaxis()->SetLabelSize(0.05);
   hframe_copy2->GetZaxis()->SetTitleSize(0.06);
   hframe_copy2->GetZaxis()->SetTitleFont(42);
   hframe_copy2->Draw("sameaxis");
   
   TH1F *hframe_copy3 = new TH1F("hframe_copy3","",1000,1.2,4.1);
   hframe_copy3->SetMinimum(0.0001);
   hframe_copy3->SetMaximum(109);
   hframe_copy3->SetDirectory(0);
   hframe_copy3->SetStats(0);
   hframe_copy3->SetLineStyle(0);
   hframe_copy3->SetMarkerStyle(20);
   hframe_copy3->GetXaxis()->SetTitle("M_{G_{Bulk}} (TeV)");
   hframe_copy3->GetXaxis()->SetNdivisions(508);
   hframe_copy3->GetXaxis()->SetLabelFont(42);
   hframe_copy3->GetXaxis()->SetLabelOffset(0.007);
   hframe_copy3->GetXaxis()->SetTitleSize(0.05);
   hframe_copy3->GetXaxis()->SetTitleOffset(1.05);
   hframe_copy3->GetXaxis()->SetTitleFont(42);
   hframe_copy3->GetYaxis()->SetTitle("#sigma #times #bf{#it{#Beta}}(G_{Bulk} #rightarrow valerieZZ) (pb)");
   hframe_copy3->GetYaxis()->SetLabelFont(42);
   hframe_copy3->GetYaxis()->SetLabelOffset(0.007);
   hframe_copy3->GetYaxis()->SetTitleSize(0.05);
   hframe_copy3->GetYaxis()->SetTitleOffset(1.15);
   hframe_copy3->GetYaxis()->SetTitleFont(42);
   hframe_copy3->GetZaxis()->SetLabelFont(42);
   hframe_copy3->GetZaxis()->SetLabelOffset(0.007);
   hframe_copy3->GetZaxis()->SetLabelSize(0.05);
   hframe_copy3->GetZaxis()->SetTitleSize(0.06);
   hframe_copy3->GetZaxis()->SetTitleFont(42);
   hframe_copy3->Draw("sameaxig");
   
   TLegend *leg = new TLegend(0.52,0.6002591,0.806734,0.9011917,NULL,"brNDC");
   leg->SetBorderSize(1);
   leg->SetTextFont(62);
   leg->SetTextSize(0.038);
   leg->SetLineColor(0);
   leg->SetLineStyle(1);
   leg->SetLineWidth(1);
   leg->SetFillColor(0);
   leg->SetFillStyle(1001);
   TLegendEntry *entry=leg->AddEntry("Graph3","Observed","Lp");
   entry->SetLineColor(1);
   entry->SetLineStyle(1);
   entry->SetLineWidth(2);
   entry->SetMarkerColor(1);
   entry->SetMarkerStyle(8);
   entry->SetMarkerSize(0.8);
   entry->SetTextFont(62);
   entry=leg->AddEntry("Graph1","Expected #pm 1 std. deviation","f");

   ci = TColor::GetColor("#00cc00");
   entry->SetFillColor(ci);
   entry->SetFillStyle(1001);

   ci = TColor::GetColor("#00cc00");
   entry->SetLineColor(ci);
   entry->SetLineStyle(1);
   entry->SetLineWidth(1);
   entry->SetMarkerColor(1);
   entry->SetMarkerStyle(21);
   entry->SetMarkerSize(1);
   entry->SetTextFont(62);
   entry=leg->AddEntry("Graph0","Expected #pm 2 std. deviation","f");

   ci = TColor::GetColor("#ffcc00");
   entry->SetFillColor(ci);
   entry->SetFillStyle(1001);

   ci = TColor::GetColor("#ffcc00");
   entry->SetLineColor(ci);
   entry->SetLineStyle(1);
   entry->SetLineWidth(1);
   entry->SetMarkerColor(1);
   entry->SetMarkerStyle(21);
   entry->SetMarkerSize(1);
   entry->SetTextFont(62);
   entry=leg->AddEntry("valerieBulkZZ_gtheory","#sigma_{TH}#times#bf{#it{#Beta}}(G_{Bulk}#rightarrowZZ) #tilde{k}=0.5","L");

   ci = TColor::GetColor("#ff0000");
   entry->SetLineColor(ci);
   entry->SetLineStyle(1);
   entry->SetLineWidth(3);
   entry->SetMarkerColor(1);
   entry->SetMarkerStyle(21);
   entry->SetMarkerSize(1);
   entry->SetTextFont(62);
   leg->Draw();
   
   leg = new TLegend(0.52,0.6002591,0.8046734,0.9011917,NULL,"brNDC");
   leg->SetTextFont(62);
   leg->SetTextSize(0.038);
   leg->SetLineColor(0);
   leg->SetLineStyle(1);
   leg->SetLineWidth(1);
   leg->SetFillColor(0);
   leg->SetFillStyle(0);
   entry=leg->AddEntry("Graph3"," ","");
   entry->SetLineColor(1);
   entry->SetLineStyle(1);
   entry->SetLineWidth(1);
   entry->SetMarkerColor(1);
   entry->SetMarkerStyle(21);
   entry->SetMarkerSize(1);
   entry->SetTextFont(62);
   entry=leg->AddEntry("Graph2"," ","L");
   entry->SetLineColor(1);
   entry->SetLineStyle(3);
   entry->SetLineWidth(4);
   entry->SetMarkerColor(1);
   entry->SetMarkerStyle(21);
   entry->SetMarkerSize(1);
   entry->SetTextFont(62);
   entry=leg->AddEntry("Graph2"," ","L");
   entry->SetLineColor(1);
   entry->SetLineStyle(3);
   entry->SetLineWidth(4);
   entry->SetMarkerColor(1);
   entry->SetMarkerStyle(21);
   entry->SetMarkerSize(1);
   entry->SetTextFont(62);
   entry=leg->AddEntry("valerieBulkZZ_grshade"," ","F");

   ci = TColor::GetColor("#ff0000");
   entry->SetFillColor(ci);
   entry->SetFillStyle(3013);
   entry->SetLineStyle(1);
   entry->SetLineWidth(1);
   entry->SetMarkerColor(1);
   entry->SetMarkerStyle(21);
   entry->SetMarkerSize(1);
   entry->SetTextFont(62);
   leg->Draw();
   c1->Modified();
   c1->cd();
   c1->SetSelected(c1);
}
