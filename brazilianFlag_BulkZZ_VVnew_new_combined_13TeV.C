void brazilianFlag_BulkZZ_VVnew_new_combined_13TeV()
{
//=========Macro generated from canvas: c1/c1
//=========  (Tue Aug  1 12:27:50 2017) by ROOT version6.06/01
   TCanvas *c1 = new TCanvas("c1", "c1",51,75,800,600);
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
   
   TH1F *hframe__1 = new TH1F("hframe__1","",1000,1.2,4.1);
   hframe__1->SetMinimum(0.0001);
   hframe__1->SetMaximum(109);
   hframe__1->SetDirectory(0);
   hframe__1->SetStats(0);
   hframe__1->SetLineStyle(0);
   hframe__1->SetMarkerStyle(20);
   hframe__1->GetXaxis()->SetTitle("M_{G_{Bulk}} (TeV)");
   hframe__1->GetXaxis()->SetNdivisions(508);
   hframe__1->GetXaxis()->SetLabelFont(42);
   hframe__1->GetXaxis()->SetLabelOffset(0.007);
   hframe__1->GetXaxis()->SetTitleSize(0.05);
   hframe__1->GetXaxis()->SetTitleOffset(1.05);
   hframe__1->GetXaxis()->SetTitleFont(42);
   hframe__1->GetYaxis()->SetTitle("#sigma #times #bf{#it{#Beta}}(G_{Bulk} #rightarrow ZZ) (pb)");
   hframe__1->GetYaxis()->SetLabelFont(42);
   hframe__1->GetYaxis()->SetLabelOffset(0.007);
   hframe__1->GetYaxis()->SetTitleSize(0.05);
   hframe__1->GetYaxis()->SetTitleOffset(1.15);
   hframe__1->GetYaxis()->SetTitleFont(42);
   hframe__1->GetZaxis()->SetLabelFont(42);
   hframe__1->GetZaxis()->SetLabelOffset(0.007);
   hframe__1->GetZaxis()->SetLabelSize(0.05);
   hframe__1->GetZaxis()->SetTitleSize(0.06);
   hframe__1->GetZaxis()->SetTitleFont(42);
   hframe__1->Draw(" ");
   
   Double_t Graph0_fx1[60] = {
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
   4.1,
   4.1,
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
   Double_t Graph0_fy1[60] = {
   0.01585676,
   0.01059784,
   0.008437859,
   0.006899073,
   0.005431509,
   0.00420888,
   0.003294995,
   0.002647557,
   0.002176638,
   0.001831162,
   0.001569543,
   0.001395269,
   0.0012348,
   0.00109921,
   0.001020695,
   0.0009041783,
   0.0008215168,
   0.0007438068,
   0.0006863901,
   0.0006223391,
   0.000581362,
   0.0005351165,
   0.0004950907,
   0.000465236,
   0.000440357,
   0.0003991848,
   0.0003800622,
   0.0003657202,
   0.0003465976,
   0.0003226943,
   0.002555485,
   0.002744555,
   0.00289586,
   0.003009423,
   0.00316084,
   0.003350112,
   0.003539239,
   0.003766201,
   0.004030994,
   0.004295942,
   0.004567743,
   0.004875571,
   0.005246837,
   0.005606241,
   0.006070476,
   0.006687498,
   0.007148553,
   0.007856072,
   0.008731768,
   0.009574443,
   0.01102233,
   0.01282406,
   0.01518568,
   0.01855074,
   0.02304723,
   0.02923519,
   0.03643718,
   0.0433427,
   0.05313478,
   0.08068738};
   TGraph *graph = new TGraph(60,Graph0_fx1,Graph0_fy1);
   graph->SetName("Graph0");
   graph->SetTitle("Graph");

   Int_t ci;      // for color index setting
   TColor *color; // for color definition with alpha
   ci = TColor::GetColor("#ffcc00");
   graph->SetFillColor(ci);

   ci = TColor::GetColor("#ffcc00");
   graph->SetLineColor(ci);
   graph->SetMarkerStyle(20);
   
   TH1F *Graph_Graph1 = new TH1F("Graph_Graph1","Graph",100,0.91,4.39);
   Graph_Graph1->SetMinimum(0.0002904249);
   Graph_Graph1->SetMaximum(0.08872385);
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
   
   Double_t Graph1_fx2[60] = {
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
   4.1,
   4.1,
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
   Double_t Graph1_fy2[60] = {
   0.0219041,
   0.01469622,
   0.01177843,
   0.009613875,
   0.007602581,
   0.005917844,
   0.00465406,
   0.003739578,
   0.003092936,
   0.002622975,
   0.002243876,
   0.002004282,
   0.001782378,
   0.001598411,
   0.001473822,
   0.001326085,
   0.001193708,
   0.001107944,
   0.001007484,
   0.0009229974,
   0.0008622239,
   0.0008021471,
   0.0007520403,
   0.0007012035,
   0.000663706,
   0.0006103608,
   0.0005811219,
   0.0005591928,
   0.0005299539,
   0.0004934054,
   0.00156201,
   0.001648843,
   0.001724581,
   0.001792212,
   0.001882386,
   0.001995103,
   0.002089204,
   0.002203459,
   0.00233727,
   0.002490893,
   0.002642271,
   0.002833558,
   0.003070585,
   0.003298398,
   0.003597282,
   0.003986725,
   0.004293397,
   0.00471389,
   0.005279557,
   0.005831174,
   0.006711884,
   0.007871567,
   0.009321801,
   0.01148787,
   0.01440755,
   0.01843847,
   0.02305385,
   0.02782509,
   0.03417633,
   0.05160457};
   graph = new TGraph(60,Graph1_fx2,Graph1_fy2);
   graph->SetName("Graph1");
   graph->SetTitle("Graph");

   ci = TColor::GetColor("#00cc00");
   graph->SetFillColor(ci);

   ci = TColor::GetColor("#00cc00");
   graph->SetLineColor(ci);
   graph->SetMarkerStyle(20);
   
   TH1F *Graph_Graph2 = new TH1F("Graph_Graph2","Graph",100,0.91,4.39);
   Graph_Graph2->SetMinimum(0.0004440649);
   Graph_Graph2->SetMaximum(0.05671568);
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
   
   Double_t Graph2_fx1001[30] = {
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
   4.1};
   Double_t Graph2_fy1001[30] = {
   0.03286906,
   0.02187941,
   0.0176334,
   0.01453632,
   0.01153914,
   0.009016514,
   0.0071183,
   0.005719617,
   0.004782998,
   0.00405868,
   0.003509197,
   0.003147038,
   0.002809855,
   0.002535114,
   0.002354034,
   0.00210427,
   0.001929434,
   0.001779575,
   0.001642204,
   0.001517322,
   0.001417416,
   0.001329998,
   0.001242581,
   0.001167651,
   0.00110521,
   0.001042769,
   0.0009928156,
   0.0009553508,
   0.0009053979,
   0.0008429566};
   Double_t Graph2_fex1001[30] = {
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
   0,
   0};
   Double_t Graph2_fey1001[30] = {
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
   0,
   0};
   gre = new TGraphErrors(30,Graph2_fx1001,Graph2_fy1001,Graph2_fex1001,Graph2_fey1001);
   gre->SetName("Graph2");
   gre->SetTitle("Graph");
   gre->SetFillColor(1);
   gre->SetLineStyle(3);
   gre->SetLineWidth(4);
   gre->SetMarkerStyle(20);
   
   TH1F *Graph_Graph1001 = new TH1F("Graph_Graph1001","Graph",100,0.91,4.39);
   Graph_Graph1001->SetMinimum(0.000758661);
   Graph_Graph1001->SetMaximum(0.03607168);
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
   
   Double_t Graph3_fx1002[30] = {
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
   4.1};
   Double_t Graph3_fy1002[30] = {
   0.03100709,
   0.01557259,
   0.02593309,
   0.01609415,
   0.00607986,
   0.00843089,
   0.008120714,
   0.006572902,
   0.006850224,
   0.006056969,
   0.005937819,
   0.005011636,
   0.002899969,
   0.002235551,
   0.002244453,
   0.00269107,
   0.002640291,
   0.002621451,
   0.002553959,
   0.002419561,
   0.002284246,
   0.001985911,
   0.001073334,
   0.0006816879,
   0.0005902711,
   0.0005811769,
   0.0005968092,
   0.0006074441,
   0.0006084506,
   0.0005934591};
   Double_t Graph3_fex1002[30] = {
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
   0,
   0};
   Double_t Graph3_fey1002[30] = {
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
   0,
   0};
   gre = new TGraphErrors(30,Graph3_fx1002,Graph3_fy1002,Graph3_fex1002,Graph3_fey1002);
   gre->SetName("Graph3");
   gre->SetTitle("Graph");
   gre->SetFillColor(1);
   gre->SetLineWidth(2);
   gre->SetMarkerStyle(8);
   gre->SetMarkerSize(0.8);
   
   TH1F *Graph_Graph1002 = new TH1F("Graph_Graph1002","Graph",100,0.91,4.39);
   Graph_Graph1002->SetMinimum(0.0005230592);
   Graph_Graph1002->SetMaximum(0.03404968);
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
   
   Double_t BulkZZ_gtheory_fx1003[11] = {
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
   Double_t BulkZZ_gtheory_fy1003[11] = {
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
   Double_t BulkZZ_gtheory_fex1003[11] = {
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
   Double_t BulkZZ_gtheory_fey1003[11] = {
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
   gre = new TGraphErrors(11,BulkZZ_gtheory_fx1003,BulkZZ_gtheory_fy1003,BulkZZ_gtheory_fex1003,BulkZZ_gtheory_fey1003);
   gre->SetName("BulkZZ_gtheory");
   gre->SetTitle("Graph");
   gre->SetFillColor(1);

   ci = TColor::GetColor("#ff0000");
   gre->SetLineColor(ci);
   gre->SetLineWidth(3);
   
   TH1F *Graph_BulkZZ_gtheory1003 = new TH1F("Graph_BulkZZ_gtheory1003","Graph",100,0.65,4.85);
   Graph_BulkZZ_gtheory1003->SetMinimum(9.403527e-07);
   Graph_BulkZZ_gtheory1003->SetMaximum(0.01127473);
   Graph_BulkZZ_gtheory1003->SetDirectory(0);
   Graph_BulkZZ_gtheory1003->SetStats(0);
   Graph_BulkZZ_gtheory1003->SetLineStyle(0);
   Graph_BulkZZ_gtheory1003->SetMarkerStyle(20);
   Graph_BulkZZ_gtheory1003->GetXaxis()->SetLabelFont(42);
   Graph_BulkZZ_gtheory1003->GetXaxis()->SetLabelOffset(0.007);
   Graph_BulkZZ_gtheory1003->GetXaxis()->SetLabelSize(0.05);
   Graph_BulkZZ_gtheory1003->GetXaxis()->SetTitleSize(0.06);
   Graph_BulkZZ_gtheory1003->GetXaxis()->SetTitleOffset(0.9);
   Graph_BulkZZ_gtheory1003->GetXaxis()->SetTitleFont(42);
   Graph_BulkZZ_gtheory1003->GetYaxis()->SetLabelFont(42);
   Graph_BulkZZ_gtheory1003->GetYaxis()->SetLabelOffset(0.007);
   Graph_BulkZZ_gtheory1003->GetYaxis()->SetLabelSize(0.05);
   Graph_BulkZZ_gtheory1003->GetYaxis()->SetTitleSize(0.06);
   Graph_BulkZZ_gtheory1003->GetYaxis()->SetTitleOffset(1.25);
   Graph_BulkZZ_gtheory1003->GetYaxis()->SetTitleFont(42);
   Graph_BulkZZ_gtheory1003->GetZaxis()->SetLabelFont(42);
   Graph_BulkZZ_gtheory1003->GetZaxis()->SetLabelOffset(0.007);
   Graph_BulkZZ_gtheory1003->GetZaxis()->SetLabelSize(0.05);
   Graph_BulkZZ_gtheory1003->GetZaxis()->SetTitleSize(0.06);
   Graph_BulkZZ_gtheory1003->GetZaxis()->SetTitleFont(42);
   gre->SetHistogram(Graph_BulkZZ_gtheory1003);
   
   gre->Draw("l");
   
   Double_t BulkZZ_grshade_fx3[22] = {
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
   Double_t BulkZZ_grshade_fy3[22] = {
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
   graph = new TGraph(22,BulkZZ_grshade_fx3,BulkZZ_grshade_fy3);
   graph->SetName("BulkZZ_grshade");
   graph->SetTitle("Graph");

   ci = TColor::GetColor("#ff0000");
   graph->SetFillColor(ci);
   graph->SetFillStyle(3013);
   graph->SetLineColor(0);
   
   TH1F *Graph_BulkZZ_grshade3 = new TH1F("Graph_BulkZZ_grshade3","Graph",100,0.65,4.85);
   Graph_BulkZZ_grshade3->SetMinimum(8.827495e-08);
   Graph_BulkZZ_grshade3->SetMaximum(0.01474076);
   Graph_BulkZZ_grshade3->SetDirectory(0);
   Graph_BulkZZ_grshade3->SetStats(0);
   Graph_BulkZZ_grshade3->SetLineStyle(0);
   Graph_BulkZZ_grshade3->SetMarkerStyle(20);
   Graph_BulkZZ_grshade3->GetXaxis()->SetLabelFont(42);
   Graph_BulkZZ_grshade3->GetXaxis()->SetLabelOffset(0.007);
   Graph_BulkZZ_grshade3->GetXaxis()->SetLabelSize(0.05);
   Graph_BulkZZ_grshade3->GetXaxis()->SetTitleSize(0.06);
   Graph_BulkZZ_grshade3->GetXaxis()->SetTitleOffset(0.9);
   Graph_BulkZZ_grshade3->GetXaxis()->SetTitleFont(42);
   Graph_BulkZZ_grshade3->GetYaxis()->SetLabelFont(42);
   Graph_BulkZZ_grshade3->GetYaxis()->SetLabelOffset(0.007);
   Graph_BulkZZ_grshade3->GetYaxis()->SetLabelSize(0.05);
   Graph_BulkZZ_grshade3->GetYaxis()->SetTitleSize(0.06);
   Graph_BulkZZ_grshade3->GetYaxis()->SetTitleOffset(1.25);
   Graph_BulkZZ_grshade3->GetYaxis()->SetTitleFont(42);
   Graph_BulkZZ_grshade3->GetZaxis()->SetLabelFont(42);
   Graph_BulkZZ_grshade3->GetZaxis()->SetLabelOffset(0.007);
   Graph_BulkZZ_grshade3->GetZaxis()->SetLabelSize(0.05);
   Graph_BulkZZ_grshade3->GetZaxis()->SetTitleSize(0.06);
   Graph_BulkZZ_grshade3->GetZaxis()->SetTitleFont(42);
   graph->SetHistogram(Graph_BulkZZ_grshade3);
   
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
   
   TH1F *hframe_copy__2 = new TH1F("hframe_copy__2","",1000,1.2,4.1);
   hframe_copy__2->SetMinimum(0.0001);
   hframe_copy__2->SetMaximum(109);
   hframe_copy__2->SetDirectory(0);
   hframe_copy__2->SetStats(0);
   hframe_copy__2->SetLineStyle(0);
   hframe_copy__2->SetMarkerStyle(20);
   hframe_copy__2->GetXaxis()->SetTitle("M_{G_{Bulk}} (TeV)");
   hframe_copy__2->GetXaxis()->SetNdivisions(508);
   hframe_copy__2->GetXaxis()->SetLabelFont(42);
   hframe_copy__2->GetXaxis()->SetLabelOffset(0.007);
   hframe_copy__2->GetXaxis()->SetTitleSize(0.05);
   hframe_copy__2->GetXaxis()->SetTitleOffset(1.05);
   hframe_copy__2->GetXaxis()->SetTitleFont(42);
   hframe_copy__2->GetYaxis()->SetTitle("#sigma #times #bf{#it{#Beta}}(G_{Bulk} #rightarrow ZZ) (pb)");
   hframe_copy__2->GetYaxis()->SetLabelFont(42);
   hframe_copy__2->GetYaxis()->SetLabelOffset(0.007);
   hframe_copy__2->GetYaxis()->SetTitleSize(0.05);
   hframe_copy__2->GetYaxis()->SetTitleOffset(1.15);
   hframe_copy__2->GetYaxis()->SetTitleFont(42);
   hframe_copy__2->GetZaxis()->SetLabelFont(42);
   hframe_copy__2->GetZaxis()->SetLabelOffset(0.007);
   hframe_copy__2->GetZaxis()->SetLabelSize(0.05);
   hframe_copy__2->GetZaxis()->SetTitleSize(0.06);
   hframe_copy__2->GetZaxis()->SetTitleFont(42);
   hframe_copy__2->Draw("sameaxis");
   
   TH1F *hframe_copy__3 = new TH1F("hframe_copy__3","",1000,1.2,4.1);
   hframe_copy__3->SetMinimum(0.0001);
   hframe_copy__3->SetMaximum(109);
   hframe_copy__3->SetDirectory(0);
   hframe_copy__3->SetStats(0);
   hframe_copy__3->SetLineStyle(0);
   hframe_copy__3->SetMarkerStyle(20);
   hframe_copy__3->GetXaxis()->SetTitle("M_{G_{Bulk}} (TeV)");
   hframe_copy__3->GetXaxis()->SetNdivisions(508);
   hframe_copy__3->GetXaxis()->SetLabelFont(42);
   hframe_copy__3->GetXaxis()->SetLabelOffset(0.007);
   hframe_copy__3->GetXaxis()->SetTitleSize(0.05);
   hframe_copy__3->GetXaxis()->SetTitleOffset(1.05);
   hframe_copy__3->GetXaxis()->SetTitleFont(42);
   hframe_copy__3->GetYaxis()->SetTitle("#sigma #times #bf{#it{#Beta}}(G_{Bulk} #rightarrow ZZ) (pb)");
   hframe_copy__3->GetYaxis()->SetLabelFont(42);
   hframe_copy__3->GetYaxis()->SetLabelOffset(0.007);
   hframe_copy__3->GetYaxis()->SetTitleSize(0.05);
   hframe_copy__3->GetYaxis()->SetTitleOffset(1.15);
   hframe_copy__3->GetYaxis()->SetTitleFont(42);
   hframe_copy__3->GetZaxis()->SetLabelFont(42);
   hframe_copy__3->GetZaxis()->SetLabelOffset(0.007);
   hframe_copy__3->GetZaxis()->SetLabelSize(0.05);
   hframe_copy__3->GetZaxis()->SetTitleSize(0.06);
   hframe_copy__3->GetZaxis()->SetTitleFont(42);
   hframe_copy__3->Draw("sameaxig");
   
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
   entry=leg->AddEntry("BulkZZ_gtheory","#sigma_{TH}#times#bf{#it{#Beta}}(G_{Bulk}#rightarrowZZ) #tilde{k}=0.5","L");

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
   entry=leg->AddEntry("BulkZZ_grshade"," ","F");

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
