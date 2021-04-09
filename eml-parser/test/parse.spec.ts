import { expect } from "chai";
import { parseEmailFromEmlHtml } from "../src/parse";

describe("parse.ts", () => {
  it("should parse eml file", async () => {
    const id = "test-id";
    const {
      html,
      recipientEmail,
      senderAddress,
      senderName,
      subject,
      receivedAt,
    } = await parseEmailFromEmlHtml(id, sampleEml);

    expect(recipientEmail).to.eq("test@prizeprofile.com");
    expect(senderAddress).to.eq("email@emailnastygal.com");
    expect(senderName).to.eq("Nasty Gal");
    expect(html).to.eq(sampleHtml);
    expect(subject).to.eq("RE: Your Winter Wardrobe");
    expect(receivedAt.toDateString()).to.eq("Sun Dec 13 2020");
  });
});

const sampleEml = `Return-Path: <email@emailnastygal.com>
Received: from omp.emailnastygal.com (omp.emailnastygal.com [199.7.207.63])
 by inbound-smtp.eu-west-1.amazonaws.com with SMTP id qep0tn36os1n6eij5od11faraurfobl6edqe9mg1
 for test@prizeprofile.com;
 Sun, 13 Dec 2020 18:01:31 +0000 (UTC)
X-SES-Spam-Verdict: PASS
X-SES-Virus-Verdict: PASS
Received-SPF: pass (spfCheck: domain of emailnastygal.com designates 199.7.207.63 as permitted sender) client-ip=199.7.207.63; envelope-from=email@emailnastygal.com; helo=omp.emailnastygal.com;
Authentication-Results: amazonses.com;
 spf=pass (spfCheck: domain of emailnastygal.com designates 199.7.207.63 as permitted sender) client-ip=199.7.207.63; envelope-from=email@emailnastygal.com; helo=omp.emailnastygal.com;
 dkim=pass header.i=@emailnastygal.com;
 dmarc=pass header.from=emailnastygal.com;
X-SES-RECEIPT: AEFBQUFBQUFBQUFGVmxzVTUvaC81QnNsKzYraWFySEIrcUcvT3djM0JPTEl5aSs0VmFuQkF6cFZ6MXhLZUxjYnFhaDJSU0Z3VU9VQnFXM1ovcS9NWjJtdmdMQzJBbUk5MU1zaVQ5dnA0Uzd0WFJocHFLbk5PTWVDUXJvdFlQbzJ3bEZUMUNEakJKUWxrRHlKNXkxU1l3MTQzeWEzTE1jU1NkZnp1U0c2S2JUWUhQbnp6S3liRjdDQVplNjAxQ0pla3ZoMGU1cm9XSXQyakRKbUovMWNCMkhMWnR6cFh0U1d2azRmaUs1QjA3S0YwSWdMZHk5aFAzeUJGL0J2bGhxWUxKemZDZGVHOFlVNEFQbDV5S0ZVWEpKTmtUUVlMYkRpV2Iya1NVaklZc3djYWZMVEtvc1NCNUE9PQ==
X-SES-DKIM-SIGNATURE: a=rsa-sha256; q=dns/txt; b=o/NHGsJPZEHDesj97JPjkke2dBR9vnO8cV0vsUOt2XpX5JImtNlKMiyeP19Ho60m+SQdKBKjqkMRjbIEASjxK+ibraGGxdulcH8tc6w7CjQH5JeYCqXrTatJM8MFHlealh/gIRzE+aMwkjT8ETh3qbxMNXR++HAWRticsjI1sWs=; c=relaxed/simple; s=shh3fegwg5fppqsuzphvschd53n6ihuv; d=amazonses.com; t=1607882493; v=1; bh=yvB3fORuZ9CSZDCDwingVjscUw6Krg4br8x3U6FfUtE=; h=From:To:Cc:Bcc:Subject:Date:Message-ID:MIME-Version:Content-Type:X-SES-RECEIPT;
DKIM-Signature: v=1; a=rsa-sha256; c=relaxed/relaxed; s=emailnastygal; d=emailnastygal.com;
 h=MIME-Version:Content-Type:Content-Transfer-Encoding:Date:To:From:Reply-To:
 Subject:List-Unsubscribe:List-Unsubscribe-Post:Message-ID;
 i=email@emailnastygal.com;
 bh=BFNa6WJ/kBSzu7K+pER31taT648zooKCeHtXg7V4EJ4=;
 b=cUIxOwqZ9la9F5SFMRbNqF/mLXcagkwVc+49UAnleT7Z+vGY4oWpsWz0JwOErvpL/iARt1a5IdoJ
   MFcNUujeHpwkZxj9p9jhAcfEovZz2ZCebqboltvHJYs69Bez2/FVE/FX0jvs9YIbVBIBEMUPahjZ
   MM4rPk/7Er/5ZTXVEz8=
Received: by omp.emailnastygal.com id hqpbfi2lr0og for <test@prizeprofile.com>; Sun, 13 Dec 2020 10:01:29 -0800 (envelope-from <email@emailnastygal.com>)
MIME-Version: 1.0
Content-Type: text/html;
	charset="UTF-8"
Content-Transfer-Encoding: quoted-printable
Date: Sun, 13 Dec 2020 10:01:29 -0800
To: test@prizeprofile.com
From: =?UTF-8?B?TmFzdHkgR2Fs?= <email@emailnastygal.com>
Reply-To: =?UTF-8?B?TmFzdHkgR2Fs?= <noreply@emailnastygal.com>
Subject: RE: Your Winter Wardrobe
Feedback-ID: 56578:98978565:oraclersys
List-Unsubscribe: <mailto:unsubscribe-AQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaG@imh.rsys5.com?subject=List-Unsubscribe>, <https://emailnastygal.com/pub/optout/UnsubscribeOneStepConfirmAction?YES=true&_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaG&_ei_=EolaGGF4SNMvxFF7KucKuWNjwOLyncDiCbA_Akyj-vMWiVKaEzA.>
List-Unsubscribe-Post: List-Unsubscribe=One-Click
X-sgxh1: lLklxijpFLijhMptLQJhu
X-rext: 6.interact5.ElZtzxWy6NfIw9kt1qCXuTnibICqk8ml2LyX3rc9TFSXWuTCkTPgaQE
X-cid: nastygal.102997285
X-ei: EsK-pXUrUBNQ-fZvSY8uCOo
Message-ID: <0.1.288.653.1D6D179FB5639EC.0@omp.emailnastygal.com>

<=21DOCTYPE html>
<html lang=3D=22en=22 xmlns=3D=22http://www=2Ew3=2Eorg/1999/xhtml=22 xmlns:=
v=3D=22urn:schemas-microsoft-com:vml=22 xmlns:o=3D=22urn:schemas-microsoft-=
com:office:office=22>

<head>
    <meta charset=3D=22utf-8=22>
    <meta name=3D=22viewport=22 content=3D=22width=3Ddevice-width, initial-=
scale=3D1=22>
    <meta http-equiv=3D=22X-UA-Compatible=22 content=3D=22IE=3Dedge=22>
    <meta name=3D=22x-apple-disable-message-reformatting=22>

    <title>Nasty Girl</title>

    <=21-- // Ooutlook DPI Scaling Fix -->
    <=21--=5Bif gte mso 9=5D>
    <xml>
        <o:OfficeDocumentSettings>
            <o:AllowPNG/>
            <o:PixelsPerInch>96</o:PixelsPerInch>
        </o:OfficeDocumentSettings>
    </xml>
    <=21=5Bendif=5D-->


    <style media=3D=22all=22 type=3D=22text/css=22>
        /*--- Fonts ---*/
        =40font-face=7B
            font-family:'Open Sans';
            font-style:normal;
            font-weight:400;
            src:local('Open Sans'), local('OpenSans'), url('http://fonts=2E=
gstatic=2Ecom/s/opensans/v10/cJZKeOuBrn4kERxqtaUH3bO3LdcAZYWl9Si6vvxL-qU=2E=
woff') format('woff');
        =7D
        =40font-face =7B
            font-family: 'NG-Grotesque';
            src: url('https://static=2Ecdn=2Eresponsys=2Enet/i5/responsysim=
ages/content/boohoo/NGGrotesque1=2Eeot');
            src: url('https://static=2Ecdn=2Eresponsys=2Enet/i5/responsysim=
ages/content/boohoo/NGGrotesque1=2Eeot?iefix') format('embedded-opentype'),
                url('https://www=2Eboohoo=2Ecom/on/demandware=2Estatic/-/Li=
brary-Sites-boohoo-content-global/default/dw79a56c45/landing-pages/global-e=
lements/fonts/NGGrotesque/ng-grotesque-1=2Eotf') format('opentype'),
                url('https://static=2Ecdn=2Eresponsys=2Enet/i5/responsysima=
ges/content/boohoo/NGGrotesque1=2Ewoff') format('woff');
            font-weight: 300;
            font-style: normal;
            mso-font-alt: 'Arial';
        =7D
        =40font-face =7B
            font-family: 'NG-Grotesque';
            src: url('https://www=2Eboohoo=2Ecom/on/demandware=2Estatic/-/L=
ibrary-Sites-boohoo-content-global/default/dw79a56c45/landing-pages/global-=
elements/fonts/NGGrotesque/ng-grotesque-2=2Eotf') format('opentype');
            font-weight: normal;
            font-style: normal;
            mso-font-alt: 'Arial';
        =7D
        =40font-face =7B
            font-family: 'NG-Grotesque';
            src: url('https://static=2Ecdn=2Eresponsys=2Enet/i5/responsysim=
ages/content/boohoo/NGGrotesque3=2Eeot');
            src: url('https://static=2Ecdn=2Eresponsys=2Enet/i5/responsysim=
ages/content/boohoo/NGGrotesque3=2Eeot?iefix') format('embedded-opentype'),
                url('https://www=2Eboohoo=2Ecom/on/demandware=2Estatic/-/Li=
brary-Sites-boohoo-content-global/default/dw79a56c45/landing-pages/global-e=
lements/fonts/NGGrotesque/ng-grotesque-3=2Eotf') format('opentype'),
                url('https://static=2Ecdn=2Eresponsys=2Enet/i5/responsysima=
ges/content/boohoo/NGGrotesque3=2Ewoff') format('woff');
            font-weight: bold;
            font-style: normal;
            mso-font-alt: 'Arial';
        =7D
    </style>

    <style type=3D=22text/css=22>
    html, body =7B
			margin: 0 auto =21important;
			padding: 0 =21important;
			height: 100% =21important;
			width: 100% =21important;
            font-family: Arial, sans-serif;
            background-color: =23EEEEEE;
        =7D
       =20
        =2Ebg_color =7B background-color: =23EEEEEE; =7D
        =2Ebg_white =7B background-color: =23FFFFFF; =7D

        =2Ewrapper =7B table-layout: fixed =21important; =7D
        =2Etb_images =7B table-layout: auto =21important; =7D

        =2Emobile-hide =7B display: none =21important; =7D

        =2Epadding =7B padding: 10px 5% 10px 5%; =7D
        =2Eicon-padding =7B padding: 10px 8px 10px 8px; text-align: center;=
 =7D
        =2Eno-padding =7B padding: 0 =21important; =7D
        =2Esection-padding =7B padding: 50px 15px 50px 15px =21important; =
=7D

        =2Ecopy_top_text =7B font-family: 'NG-Grotesque', Helvetica, Arial,=
 sans-serif; font-size: 11px; line-height: 100%; font-weight: 300; color: =
=23999999; =7D
        =2Ecopy_top_text a =7B color:=23999999; text-decoration:none; =7D
        =2Ecopy_top_text a:hover =7B color:=23999999; text-decoration:under=
line; =7D

        =2Elogo =7B display: inline-block; font-size: 0; line-height: 0px; =
margin: 0; padding: 0; =7D
        =2Elogo img =7B margin: 0 auto =21important; max-width: 200px; marg=
in: 0; padding: 0; =7D

        =2Eheader_button =7B font-family: 'NG-Grotesque', Helvetica, Arial,=
 sans-serif; font-size: 16px; font-weight: 300; color: =23000000; margin: 0=
; padding: 0; =7D
        =2Eheader_button a =7B color: =23000000; text-decoration:none; =7D
        =2Eheader_button a:hover =7B color: =23000000; text-decoration:unde=
rline; =7D

        =2Ecopy_heading =7B font-family: 'NG-Grotesque', Helvetica, Arial, =
sans-serif; font-size: 32px; font-weight: 300; color: =23000000; margin: 0;=
 padding: 0; =7D
        =2Ecopy_heading a =7B color:=23000000; text-decoration:none; =7D
        =2Ecopy_heading a:hover =7B color:=23000000; text-decoration:underl=
ine; =7D

        =2Ecopy_body=7B font-family: 'NG-Grotesque', Helvetica, Arial, sans=
-serif; font-size: 18px; font-weight: 300; color: =23000000; margin: 0; pad=
ding: 0; =7D
        =2Ecopy_body a =7B color:=23000000; text-decoration:none; =7D
        =2Ecopy_body a:hover =7B color:=23000000; text-decoration:underline=
; =7D

        =2Ecta_button =7B font-family: 'NG-Grotesque', Helvetica, Arial, sa=
ns-serif; font-size: 16px; font-weight: 300; color: =23000000; margin: 0; p=
adding: 0; =7D
        =2Ecta_button a =7B color:=23000000; text-decoration:none; =7D
        =2Ecta_button a:hover =7B color:=23000000; text-decoration:none; =7D

        =2Ecopy_legal =7B font-family: 'NG-Grotesque', Helvetica, Arial, sa=
ns-serif; font-size:13px; line-height:18px; font-weight: 300; color:=236666=
66; margin: 0; padding: 0; =7D
        =2Ecopy_legal a =7B color:=23666666; text-decoration:none; =7D
        =2Ecopy_legal a:hover =7B color:=23666666; text-decoration:underlin=
e; =7D


        /* MOBILE STYLES */
        =40media screen and (max-width: 700px) =7B
            =2Eimg-max =7B max-width: 100% =21important; width: 100% =21imp=
ortant; height: auto =21important; =7D
            =2Eimg-padded =7B max-width: 96% =21important; width: 96% =21im=
portant; height: auto =21important; =7D
        =7D

        =40media screen and (max-width: 480px) =7B
            =2Ewrapper =7B width: 100% =21important; max-width: 100% =21imp=
ortant; =7D
            =2Eresponsive-table =7B width: 100% =21important; =7D

            =2Ecopy_top_text =7B font-size: 10px; =7D
            =2Elogo img =7B margin: 0 auto =21important; max-width: 68%; =7D
            =2Eheader_buttons =7B width: 90% =21important; max-width: 90% =
=21important; =7D
            =2Eheader_button =7B font-size: 12px; =7D

            =2Ecopy_heading =7B font-size: 24px; =7D
            =2Ecopy_body =7B font-size: 14px; =7D
            =2Ecta_button =7B font-size: 13px; =7D

            =2Eicon-padding =7B padding: 10px 5% 10px 5%; text-align: cente=
r; =7D
            =2Esocial_icons =7B width: 70% =21important; max-width: 70% =21=
important; =7D
            =2Eapp_icons =7B width: 60% =21important; max-width: 60% =21imp=
ortant; =7D
            =2Ecopy_legal =7B font-size: 11px; line-height: 15px; =7D
        =7D
    </style>

    <style type=3D=22text/css=22>
        /* CSS Resets */
        =23MessageViewBody, =23MessageWebViewDiv=7B width: 100% =21importan=
t; =7D
        * =7B -ms-text-size-adjust: 100%; -webkit-text-size-adjust: 100%; =
=7D
        *=5Bx-apple-data-detectors=5D, =2Eunstyle-auto-detected-links *, =
=2EaBn =7B=20
            border-bottom: 0 =21important;=20
            cursor: default =21important;=20
            color: inherit =21important;=20
            text-decoration: none =21important;=20
            font-size: inherit =21important;=20
            font-family: inherit =21important;=20
            font-weight: inherit =21important;=20
            line-height: inherit =21important;=20
        =7D

        div=5Bstyle*=3D=22margin: 16px 0=22=5D =7B margin: 0 auto =21import=
ant; font-size:100% =21important; =7D
        body, table, td, a =7B-webkit-text-size-adjust: 100%; -ms-text-size=
-adjust: 100%;=7D
        table, td =7B mso-table-lspace: 0pt =21important; mso-table-rspace:=
 0pt =21important; =7D
        table =7B border-spacing: 0 =21important; border-collapse: collapse=
 =21important; margin: 0 auto =21important; =7D
        img =7B -ms-interpolation-mode:bicubic; =7D
        a =7B text-decoration: none; =7D
        =2Ea6S =7B display: none =21important; opacity: 0=2E01 =21important=
; =7D
        =2Eim =7B color: inherit =21important; =7D
        img=2Eg-img + div =7B display: none =21important; =7D
       =20
        /* RESET STYLES */
        img =7Bborder: 0; height: auto; line-height: 100%; outline: none; t=
ext-decoration: none;=7D
        table =7Bborder-collapse: collapse =21important;=7D
        body =7Bheight: 100% =21important; margin: 0 =21important; padding:=
 0 =21important; width: 100% =21important; =7D

        /* iPhone Fixes */
        =40media only screen and (min-device-width: 320px) and (max-device-=
width: 374px) =7Bu =7E div =2Eem_container =7B min-width: 320px =21importan=
t; =7D=7D
        =40media only screen and (min-device-width: 375px) and (max-device-=
width: 413px) =7Bu =7E div =2Eem_container =7B min-width: 375px =21importan=
t; =7D=7D
        =40media only screen and (min-device-width: 414px) =7Bu =7E div =2E=
em_container =7B min-width: 414px =21important; =7D=7D
        =40media screen and (max-width: 480px) =7Bu + =2Ebody-wrap =2Efull-=
wrap =7B width:100% =21important; width:100vw =21important; =7D=7D
    </style>

    <=21--=5Bif (gte mso 9)=7C(IE)=5D>
	<style type=3D=22text/css=22>
		table =7Bborder-collapse: collapse =21important;=7D
		h1, h2, h3, h4, h5, h6, p, a =7Bfont-family: Arial, sans-serif =21importa=
nt;=7D
	</style>
	<=21=5Bendif=5D-->
</head>

<body class=3D=22bg_color=22 style=3D=22background-color:=23EEEEEE; margin:=
0 =21important; padding:0 =21important;=22>
    <=21-- // HIDDEN TEXT -->
    <div style=3D=22display:none; font-size:1px; line-height:1px; font-fami=
ly:Arial, sans-serif; max-height:0px; max-width:0px; opacity:0; overflow:hi=
dden;=22>
        Handpicked just for you=2E=20
        &zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&=
nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbs=
p;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&=
zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwn=
j;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&=
nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbs=
p;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&=
zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwn=
j;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&=
nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbs=
p;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&=
zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwn=
j;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&=
nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbs=
p;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&=
zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwn=
j;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&=
nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbs=
p;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&=
zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwn=
j;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&=
nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbs=
p;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&=
zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwn=
j;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&=
nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbs=
p;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&=
zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwn=
j;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&=
nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbs=
p;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&=
zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwn=
j;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&=
nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbs=
p;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&=
zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwn=
j;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&=
nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbs=
p;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&=
zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwn=
j;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&=
nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbs=
p;&zwnj;&nbsp;&zwnj;&nbsp;
    </div>

    <=21-- =21 WRAPPER -->
    <table border=3D=220=22 cellpadding=3D=220=22 cellspacing=3D=220=22 wid=
th=3D=22100%=22 class=3D=22wrapper=22>

        <=21-- =40 HEADER TEXT -->
        <tr>
            <td align=3D=22center=22>
                <=21--=5Bif (gte mso 9)=7C(IE)=5D>
                <table align=3D=22center=22 border=3D=220=22 cellspacing=3D=
=220=22 cellpadding=3D=220=22 width=3D=22700=22>
                    <tr>
                        <td align=3D=22center=22 valign=3D=22top=22 width=
=3D=22700=22>
                <=21=5Bendif=5D-->

                <table class=3D=22wrapper=22 border=3D=220=22 cellpadding=
=3D=220=22 cellspacing=3D=220=22 width=3D=22100%=22 style=3D=22width:100%; =
max-width:700px;=22>
                    <tr>
                        <td class=3D=22copy_top_text=22 align=3D=22center=
=22 style=3D=22padding:10px 0 8px 0; text-align:center;=22>
                            <a href=3D=22https://emailnastygal=2Ecom/pub/cc=
?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJ=
D2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABWRY&amp;_ei_=3DEq2tf9zs59idfPO1Sc_9BbktrK=
mOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=2E&amp;_di_=3Dm0oj3n5jg9cvb3rkosk0qeke=
gvb7ej69ur7c2tualdrnareu4kng=22 style=3D=22font-family:'NG-Grotesque', Helv=
etica, Arial, sans-serif; font-weight:300; color:=23999999; text-decoration=
:none;=22>Handpicked just for you=2E </a>
                        </td>
                    </tr>
                </table>

                <=21--=5Bif (gte mso 9)=7C(IE)=5D>
                        </td>
                    </tr>
                </table>
                <=21=5Bendif=5D-->
            </td>
        </tr>
        <=21-- * HEADER TEXT -->

        <=21-- =40 HEADER -->
        <tr>
            <td align=3D=22center=22 style=3D=22padding:0=22 class=3D=22no-=
padding=22>
                <=21--=5Bif (gte mso 9)=7C(IE)=5D>
                <table align=3D=22center=22 border=3D=220=22 cellspacing=3D=
=220=22 cellpadding=3D=220=22 width=3D=22700=22>
                    <tr>
                        <td align=3D=22center=22 valign=3D=22top=22 width=
=3D=22700=22>
                <=21=5Bendif=5D-->

                <table class=3D=22responsive-table  bg_white=22 bgcolor=3D=
=22=23FFFFFF=22 border=3D=220=22 cellpadding=3D=220=22 cellspacing=3D=220=
=22 width=3D=22100%=22 style=3D=22width:100%; max-width:700px; background-c=
olor: =23FFFFFF;=22>
                    <=21-- =23 LOGO -->
                    <tr>
                        <td align=3D=22center=22>
                            <=21--=5Bif (gte mso 9)=7C(IE)=5D>
                            <table align=3D=22center=22 border=3D=220=22 ce=
llspacing=3D=220=22 cellpadding=3D=220=22 width=3D=22200=22>
                                <tr>
                                    <td align=3D=22center=22 valign=3D=22to=
p=22 width=3D=22200=22>
                            <=21=5Bendif=5D-->

                            <table class=3D=22wrapper=22 align=3D=22center=
=22 width=3D=22100%=22 border=3D=220=22 cellspacing=3D=220=22 cellpadding=
=3D=220=22 style=3D=22width:100%; max-width:700px;=22>
                                <tr>
                                    <td align=3D=22center=22 style=3D=22pad=
ding:3% 0 1% 0;=22>
                                        <a href=3D=22https://emailnastygal=
=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8z=
bsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABWRY&amp;_ei_=3DEq2tf9zs59idf=
PO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=2E&amp;_di_=3Dm0oj3n5jg9c=
vb3rkosk0qekegvb7ej69ur7c2tualdrnareu4kng=22 target=3D=22_blank=22 class=3D=
=22logo=22 style=3D=22display:inline-block; font-size:0px; line-height:0px;=
=22>
                                            <img src=3D=22https://i1=2Eadis=
=2Ews/i/boohooamplience/ng_em_logo=2Epng=22 alt=3D=22Nasty Gal=22 border=3D=
=220=22 width=3D=22200=22 style=3D=22width:100%; max-width:200px; display:i=
nline-block; border:0; font-size:0px; line-height:0px;=22>
                                        </a>
                                    </td>
                                </tr>
                            </table>
                           =20
                            <=21--=5Bif (gte mso 9)=7C(IE)=5D>
                                    </td>
                                </tr>
                            </table>
                            <=21=5Bendif=5D-->
                        </td>
                    </tr>
                    <=21-- ? END: LOGO -->

                    <=21-- =23 HEADER BUTTONS -->
                    <tr>
                        <td align=3D=22center=22>
                            <=21--=5Bif (gte mso 9)=7C(IE)=5D>
                            <table align=3D=22center=22 border=3D=220=22 ce=
llspacing=3D=220=22 cellpadding=3D=220=22 width=3D=22400=22>
                                <tr>
                                    <td align=3D=22center=22 valign=3D=22to=
p=22 width=3D=22400=22>
                            <=21=5Bendif=5D-->

                            <div style=3D=22display:block;=22>
                                <table class=3D=22wrapper=22 align=3D=22cen=
ter=22 border=3D=220=22 cellpadding=3D=220=22 cellspacing=3D=220=22 width=
=3D=22100%=22 style=3D=22width:100%; max-width:700px;=22>
                                    <tr>
                                        <td align=3D=22center=22 valign=3D=
=22top=22 style=3D=22padding:2% 0 3% 0; font-size:0;=22>
                                            <div class=3D=22header_buttons=
=22 style=3D=22display:inline-block; margin: 0; width:100%; max-width:400px=
; vertical-align:top;=22>
                                                <table class=3D=22tb_images=
=22 align=3D=22center=22 border=3D=220=22 cellpadding=3D=220=22 cellspacing=
=3D=220=22 width=3D=22100%=22 style=3D=22width:100%; table-layout:auto;=22>
                                                    <tr>
                                                        <td align=3D=22cent=
er=22>
                                                            <div style=3D=
=22border-right:solid 1px =23444444; font-size:16px; font-weight:300;=22>
                                                                <a href=3D=
=22https://emailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf=
4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABWTY&=
amp;_ei_=3DEq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=
=2E&amp;_di_=3Dvgvj54lpd73i1f5m9ak08ouopkkrti8c04uh1aidtva7uq2qknt0=22 targ=
et=3D=22_blank=22 class=3D=22header_button=22 style=3D=22display:inline-blo=
ck; margin:0; padding:0; color:=23000000; box-sizing:border-box; cursor:poi=
nter; text-align:center; text-decoration:none;=22>Shop New</a>
                                                            </div>
                                                        </td>
                                                        <td align=3D=22cent=
er=22>
                                                            <div style=3D=
=22border-right:solid 1px =23444444; font-size:16px; font-weight:300;=22>
                                                                <a href=3D=
=22https://emailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf=
4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABWWY&=
amp;_ei_=3DEq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=
=2E&amp;_di_=3Dsimruh5rdkjb6msanr54cqdb0vp181bgea5nhmo2lvi8lrrd5jtg=22 targ=
et=3D=22_blank=22 class=3D=22header_button=22 style=3D=22display:inline-blo=
ck; margin:0; padding:0; color:=23000000; box-sizing:border-box; cursor:poi=
nter; text-align:center; text-decoration:none;=22>Shop Dresses</a>
                                                            </div>
                                                        </td>
                                                        <td align=3D=22cent=
er=22>
                                                            <div style=3D=
=22font-size:16px; font-weight:300;=22>
                                                                <a href=3D=
=22https://emailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf=
4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABWAY&=
amp;_ei_=3DEq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=
=2E&amp;_di_=3D3k42p9u5891tup5tm6i7bqegaiks1okr2v136jp00feotfedbv10=22 targ=
et=3D=22_blank=22 class=3D=22header_button=22 style=3D=22display:inline-blo=
ck; margin:0; padding:0; color:=23000000; box-sizing:border-box; cursor:poi=
nter; text-align:center; text-decoration:none;=22>Shop Shoes</a>
                                                            </div>
                                                        </td>
                                                    </tr>
                                                </table>
                                            </div>
                                        </td>
                                    </tr>
                                </table>
                            </div>

                            <=21--=5Bif (gte mso 9)=7C(IE)=5D>
                                    </td>
                                </tr>
                            </table>
                            <=21=5Bendif=5D-->
                        </td>
                    </tr>
                    <=21-- ? END: FOOTER BUTTONS -->=20

                </table>

                <=21--=5Bif (gte mso 9)=7C(IE)=5D>
                        </td>
                    </tr>
                </table>
                <=21=5Bendif=5D-->
            </td>
        </tr>
        <=21-- * END: HEADER -->

        <=21-- =40 EMAIL CONTENT -->
        <tr>
            <td align=3D=22center=22 style=3D=22padding:0=22 class=3D=22no-=
padding=22>
                <=21--=5Bif (gte mso 9)=7C(IE)=5D>
                <table align=3D=22center=22 border=3D=220=22 cellspacing=3D=
=220=22 cellpadding=3D=220=22 width=3D=22700=22>
                    <tr>
                        <td align=3D=22center=22 valign=3D=22top=22 width=
=3D=22700=22>
                <=21=5Bendif=5D-->

                <table class=3D=22responsive-table  bg_white=22 bgcolor=3D=
=22=23FFFFFF=22 border=3D=220=22 cellpadding=3D=220=22 cellspacing=3D=220=
=22 width=3D=22100%=22 style=3D=22width:100%; max-width:700px; background-c=
olor: =23FFFFFF;=22>
                    <=21-- =23 CONTENT IMAGES -->
                    <tr>
                        <td align=3D=22center=22 style=3D=22padding:0=22 cl=
ass=3D=22no-padding=22>





							<=21-- =23 HERO IMAGE -->
                            <table class=3D=22wrapper=22 width=3D=22100%=22=
 border=3D=220=22 cellspacing=3D=220=22 cellpadding=3D=220=22>
                                <tr>
                                    <td class=3D=22no-padding=22 align=3D=
=22center=22>
                                        <a href=3D=22https://emailnastygal=
=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8z=
bsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABWCY&amp;_ei_=3DEq2tf9zs59idf=
PO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=2E&amp;_di_=3Dcqi7s7mk1n7=
3plv2pcknjm3gh3fquh2p0euphe2hh7vma8v8dhu0=22 target=3D=22_blank=22>
                                            <img src=3D=22https://static=2E=
cdn=2Eresponsys=2Enet/i5/responsysimages/nastygal/contentlibrary/2020/12dec=
ember/gbp/201213_gbp_wintershop_vs/uk/UK_Email_Banner=2Ejpg=22 alt=3D=22Nas=
ty Gal=22 border=3D=220=22 width=3D=22700=22 style=3D=22width:100%; max-wid=
th:700px; display:block; font-size:0;=22 class=3D=22img-max=22>
                                        </a>
                                    </td>
                                </tr>
                            </table>
                            <=21-- ? END: HERO IMAGE -->
						=09
							<=21-- =23 HERO IMAGE -->
                            <table class=3D=22wrapper=22 width=3D=22100%=22=
 border=3D=220=22 cellspacing=3D=220=22 cellpadding=3D=220=22>
                                <tr>
                                    <td class=3D=22no-padding=22 align=3D=
=22center=22><a href=3D=22https://emailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X=
%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2yn=
HzaGVXtpKX%3DTSWWABYRY&amp;_ei_=3DEq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63=
Ka206J-yGF-U7usJq_prM=2E&amp;_di_=3Dbracm8s9seoo0bvhgpoa7unbsjjar6i14an5esr=
mphpvq2qi76d0=22 target=3D=22_blank=22><img alt=3D=22Nasty Gal=22 border=3D=
=220=22 class=3D=22img-max=22 src=3D=22https://static=2Ecdn=2Eresponsys=2En=
et/i5/responsysimages/content/nastygal/WINTERSHOPHERO=2Egif?imbypass=3Dtrue=
=22 style=3D=22width:100%; max-width:700px; display:block; font-size:0;=22 =
width=3D=22700=22 /> </a></td>
                                </tr>
                            </table>
                            <=21-- ? END: HERO IMAGE -->
						=09
							  <=21-- =23 2 IMAGES -->
                            <table class=3D=22wrapper=22 width=3D=22100%=22=
 border=3D=220=22 cellspacing=3D=220=22 cellpadding=3D=220=22 style=3D=22wi=
dth:100%; max-width:700px;=22>
                                <tr>
                                    <td align=3D=22center=22 style=3D=22pad=
ding:0;=22>

                                        <table class=3D=22tb_images=22 widt=
h=3D=22100%=22 border=3D=220=22 cellspacing=3D=220=22 cellpadding=3D=220=22=
 style=3D=22width:100%; max-width:700px;=22>
                                            <tr valign=3D=22top=22>
                                                <td class=3D=22no-padding=
=22 align=3D=22center=22>
                                                    <a href=3D=22https://em=
ailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb=
8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABYTY&amp;_ei_=3DEq=
2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=2E&amp;_di_=3D=
ooam4b0t3ipvmi10jnevk95mdghbqndt3pfgiei76khl6rdtsr8g=22 target=3D=22_blank=
=22>
                                                        <img src=3D=22https=
://static=2Ecdn=2Eresponsys=2Enet/i5/responsysimages/nastygal/contentlibrar=
y/2020/12december/gbp/201213_gbp_wintershop_vs/uk/Winter_Shop2_02=2Ejpg=22 =
alt=3D=22Nasty Gal=22 border=3D=220=22 width=3D=22350=22 style=3D=22width:1=
00%; max-width:350px; display:block; font-size:0;=22 class=3D=22no-padding=
=22>
                                                    </a>
                                                </td>
                                                <td class=3D=22no-padding=
=22 align=3D=22center=22>
                                                    <a href=3D=22https://em=
ailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb=
8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABYWY&amp;_ei_=3DEq=
2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=2E&amp;_di_=3D=
b3ngb0htt8hm3t1m2js8vfkaljaoq1qle8hqephkacjign9ll4p0=22 target=3D=22_blank=
=22>
                                                        <img src=3D=22https=
://static=2Ecdn=2Eresponsys=2Enet/i5/responsysimages/nastygal/contentlibrar=
y/2020/12december/gbp/201213_gbp_wintershop_vs/uk/Winter_Shop2_03=2Ejpg=22 =
alt=3D=22Nasty Gal=22 border=3D=220=22 width=3D=22350=22 style=3D=22width:1=
00%; max-width:350px; display:block; font-size:0;=22 class=3D=22no-padding=
=22>
                                                    </a>
                                                </td>
                                            </tr>
                                        </table>

                                    </td>
                                </tr>
                            </table>
                            <=21-- ? END: 2 IMAGES -->
						=09
							  <=21-- =23 2 IMAGES -->
                            <table class=3D=22wrapper=22 width=3D=22100%=22=
 border=3D=220=22 cellspacing=3D=220=22 cellpadding=3D=220=22 style=3D=22wi=
dth:100%; max-width:700px;=22>
                                <tr>
                                    <td align=3D=22center=22 style=3D=22pad=
ding:0;=22>

                                        <table class=3D=22tb_images=22 widt=
h=3D=22100%=22 border=3D=220=22 cellspacing=3D=220=22 cellpadding=3D=220=22=
 style=3D=22width:100%; max-width:700px;=22>
                                            <tr valign=3D=22top=22>
                                                <td class=3D=22no-padding=
=22 align=3D=22center=22>
                                                    <a href=3D=22https://em=
ailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb=
8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABYAY&amp;_ei_=3DEq=
2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=2E&amp;_di_=3D=
3tgl7vgau5s27qhg53m79epiusdh6r62eaolbgptau01cn96nntg=22 target=3D=22_blank=
=22>
                                                        <img src=3D=22https=
://static=2Ecdn=2Eresponsys=2Enet/i5/responsysimages/nastygal/contentlibrar=
y/2020/12december/gbp/201213_gbp_wintershop_vs/uk/Winter_Shop2_04=2Ejpg=22 =
alt=3D=22Nasty Gal=22 border=3D=220=22 width=3D=22350=22 style=3D=22width:1=
00%; max-width:350px; display:block; font-size:0;=22 class=3D=22no-padding=
=22>
                                                    </a>
                                                </td>
                                                <td class=3D=22no-padding=
=22 align=3D=22center=22>
                                                    <a href=3D=22https://em=
ailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb=
8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABYCY&amp;_ei_=3DEq=
2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=2E&amp;_di_=3D=
5og5cd26u4pm2m89fnekc98327jqqo3arq6od8g8vfro93d59mng=22 target=3D=22_blank=
=22>
                                                        <img src=3D=22https=
://static=2Ecdn=2Eresponsys=2Enet/i5/responsysimages/nastygal/contentlibrar=
y/2020/12december/gbp/201213_gbp_wintershop_vs/uk/Winter_Shop2_05=2Ejpg=22 =
alt=3D=22Nasty Gal=22 border=3D=220=22 width=3D=22350=22 style=3D=22width:1=
00%; max-width:350px; display:block; font-size:0;=22 class=3D=22no-padding=
=22>
                                                    </a>
                                                </td>
                                            </tr>
                                        </table>

                                    </td>
                                </tr>
                            </table>
                            <=21-- ? END: 2 IMAGES -->


							<=21-- =23 HERO IMAGE -->
                            <table class=3D=22wrapper=22 width=3D=22100%=22=
 border=3D=220=22 cellspacing=3D=220=22 cellpadding=3D=220=22>
                                <tr>
                                    <td class=3D=22no-padding=22 align=3D=
=22center=22><a href=3D=22https://emailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X=
%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2yn=
HzaGVXtpKX%3DTSWWABARY&amp;_ei_=3DEq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63=
Ka206J-yGF-U7usJq_prM=2E&amp;_di_=3D6nn5bnaa8a2b4ld3tftuv9mqehv1ehkhbbl711c=
2mhusmb1n3t9g=22 target=3D=22_blank=22><img alt=3D=22Nasty Gal=22 border=3D=
=220=22 class=3D=22img-max=22 src=3D=22https://static=2Ecdn=2Eresponsys=2En=
et/i5/responsysimages/content/nastygal/WINTERSHOP2=2Egif?imbypass=3Dtrue=22=
 style=3D=22width:100%; max-width:700px; display:block; font-size:0;=22 wid=
th=3D=22700=22 /> </a></td>
                                </tr>
                            </table>
                            <=21-- ? END: HERO IMAGE -->


							  <=21-- =23 2 IMAGES -->
                            <table class=3D=22wrapper=22 width=3D=22100%=22=
 border=3D=220=22 cellspacing=3D=220=22 cellpadding=3D=220=22 style=3D=22wi=
dth:100%; max-width:700px;=22>
                                <tr>
                                    <td align=3D=22center=22 style=3D=22pad=
ding:0;=22>

                                        <table class=3D=22tb_images=22 widt=
h=3D=22100%=22 border=3D=220=22 cellspacing=3D=220=22 cellpadding=3D=220=22=
 style=3D=22width:100%; max-width:700px;=22>
                                            <tr valign=3D=22top=22>
                                                <td class=3D=22no-padding=
=22 align=3D=22center=22>
                                                    <a href=3D=22https://em=
ailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb=
8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABATY&amp;_ei_=3DEq=
2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=2E&amp;_di_=3D=
3ct5booibqin3htuquqpj3jh3t7ata7apod1mtb8s5p1g155svqg=22 target=3D=22_blank=
=22>
                                                        <img src=3D=22https=
://static=2Ecdn=2Eresponsys=2Enet/i5/responsysimages/nastygal/contentlibrar=
y/2020/12december/gbp/201213_gbp_wintershop_vs/uk/New_Winter_Shop2_07=2Ejpg=
=22 alt=3D=22Nasty Gal=22 border=3D=220=22 width=3D=22350=22 style=3D=22wid=
th:100%; max-width:350px; display:block; font-size:0;=22 class=3D=22no-padd=
ing=22>
                                                    </a>
                                                </td>
                                                <td class=3D=22no-padding=
=22 align=3D=22center=22>
                                                    <a href=3D=22https://em=
ailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb=
8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABAWY&amp;_ei_=3DEq=
2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=2E&amp;_di_=3D=
pnm8f70h2du6o0oriknj4mubad459a16fa86kb3qqhe7ch737c20=22 target=3D=22_blank=
=22>
                                                        <img src=3D=22https=
://static=2Ecdn=2Eresponsys=2Enet/i5/responsysimages/nastygal/contentlibrar=
y/2020/12december/gbp/201213_gbp_wintershop_vs/uk/New_Winter_Shop2_08=2Ejpg=
=22 alt=3D=22Nasty Gal=22 border=3D=220=22 width=3D=22350=22 style=3D=22wid=
th:100%; max-width:350px; display:block; font-size:0;=22 class=3D=22no-padd=
ing=22>
                                                    </a>
                                                </td>
                                            </tr>
                                        </table>

                                    </td>
                                </tr>
                            </table>
                            <=21-- ? END: 2 IMAGES -->
						=09
							<=21-- =23 HERO IMAGE -->
                            <table class=3D=22wrapper=22 width=3D=22100%=22=
 border=3D=220=22 cellspacing=3D=220=22 cellpadding=3D=220=22>
                                <tr>
                                    <td class=3D=22no-padding=22 align=3D=
=22center=22>
                                        <a href=3D=22https://emailnastygal=
=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8z=
bsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABAAY&amp;_ei_=3DEq2tf9zs59idf=
PO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=2E&amp;_di_=3Dhaqcb03ujkp=
30onoj5q5kujvnvmsf894khj93vdvnnp95vhm7adg=22 target=3D=22_blank=22>
                                            <img src=3D=22https://static=2E=
cdn=2Eresponsys=2Enet/i5/responsysimages/nastygal/contentlibrary/2020/12dec=
ember/gbp/201213_gbp_wintershop_vs/uk/Winter_Shop2_08=2Ejpg=22 alt=3D=22Nas=
ty Gal=22 border=3D=220=22 width=3D=22700=22 style=3D=22width:100%; max-wid=
th:700px; display:block; font-size:0;=22 class=3D=22img-max=22>
                                        </a>
                                    </td>
                                </tr>
                            </table>
                            <=21-- ? END: HERO IMAGE -->
						=09
							<=21-- =23 HERO IMAGE -->
                            <table class=3D=22wrapper=22 width=3D=22100%=22=
 border=3D=220=22 cellspacing=3D=220=22 cellpadding=3D=220=22>
                                <tr>
                                    <td class=3D=22no-padding=22 align=3D=
=22center=22><a href=3D=22https://emailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X=
%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2yn=
HzaGVXtpKX%3DTSWWABACY&amp;_ei_=3DEq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63=
Ka206J-yGF-U7usJq_prM=2E&amp;_di_=3Dcslqk6dejuj4bhfjvvgm3mu276qlbg9na78kfit=
von88tekd21g0=22 target=3D=22_blank=22><img alt=3D=22Nasty Gal=22 border=3D=
=220=22 class=3D=22img-max=22 src=3D=22https://static=2Ecdn=2Eresponsys=2En=
et/i5/responsysimages/nastygal/contentlibrary/2020/12december/gbp/201213_gb=
p_wintershop_vs/uk/New_UK_Promo_Postcard_02=2Egif=22 style=3D=22width:100%;=
 max-width:700px; display:block; font-size:0;=22 width=3D=22700=22 /> </a><=
/td>
                                </tr>
                            </table>
                            <=21-- ? END: HERO IMAGE -->
						=09

                        </td>
                    </tr>
                    <=21-- ? END: CONTENT IMAGES -->
                </table>

                <=21--=5Bif (gte mso 9)=7C(IE)=5D>
                        </td>
                    </tr>
                </table>
                <=21=5Bendif=5D-->
            </td>
        </tr>
        <=21-- * END: EMAIL CONTENT -->


        <=21-- =40 FOOTER -->
        <tr>
            <td align=3D=22center=22 style=3D=22padding:0=22 class=3D=22no-=
padding=22>
                <=21--=5Bif (gte mso 9)=7C(IE)=5D>
                <table align=3D=22center=22 border=3D=220=22 cellspacing=3D=
=220=22 cellpadding=3D=220=22 width=3D=22700=22>
                    <tr>
                        <td align=3D=22center=22 valign=3D=22top=22 width=
=3D=22700=22>
                <=21=5Bendif=5D-->

                <=21-- =23 HR BAR -->
                <table class=3D=22responsive-table  bg_white=22 border=3D=
=220=22 cellpadding=3D=220=22 cellspacing=3D=220=22 width=3D=22100%=22 styl=
e=3D=22width:100%; max-width:700px; background-color: =23FFFFFF;=22>
                    <tr>
                        <td align=3D=22center=22 style=3D=22margin:0; paddi=
ng: 50px 5% 0 5%; text-align:center;=22>
                            <table align=3D=22center=22 cellspacing=3D=220=
=22 cellpadding=3D=220=22 border=3D=220=22 width=3D=2296%=22 style=3D=22mar=
gin:0; padding:0; border-spacing:0; width:96%; max-width:700px; text-align:=
center;=22>
                                <tr>
                                    <td width=3D=2296%=22 height=3D=222=22 =
style=3D=22background-color: =23CCCCCC; border-collapse:collapse; mso-table=
-lspace:0pt; mso-table-rspace:0pt; mso-line-height-rule:exactly; line-heigh=
t:2px; width:96%; max-width:700px;=22>
                                        <=21--=5Bif gte mso 15=5D>&nbsp;<=
=21=5Bendif=5D-->
                                    </td>
                                </tr>
                            </table>
                        </td>
                    </tr>
                </table>
                <=21-- ? END: HR -->

                <table bgcolor=3D=22=23FFFFFF=22 border=3D=220=22 cellpaddi=
ng=3D=220=22 cellspacing=3D=220=22 width=3D=22100%=22 style=3D=22width:100%=
; max-width:700px;=22 class=3D=22responsive-table=22>
                    <tr>
                        <td align=3D=22center=22 height=3D=22100%=22 valign=
=3D=22top=22 width=3D=22100%=22>

                            <=21-- =23 APP ICONS -->
                            <div style=3D=22display:block;=22>
                                <table align=3D=22center=22 border=3D=220=
=22 cellpadding=3D=220=22 cellspacing=3D=220=22 width=3D=22100%=22 style=3D=
=22width:100%; max-width:700px;=22>
                                    <tr>
                                        <td align=3D=22center=22 valign=3D=
=22center=22 style=3D=22padding: 30px 1% 0 1%; font-size:0;=22>
                                            <=21--=5Bif (gte mso 9)=7C(IE)=
=5D>
                                            <table align=3D=22center=22 bor=
der=3D=220=22 cellspacing=3D=220=22 cellpadding=3D=220=22 width=3D=22300=22>
                                                <tr>
                                                    <td align=3D=22center=
=22 valign=3D=22center=22 width=3D=22300=22>
                                            <=21=5Bendif=5D-->

                                            <table align=3D=22center=22 bor=
der=3D=220=22 cellpadding=3D=220=22 cellspacing=3D=220=22 width=3D=22100%=
=22 style=3D=22width: 100%; max-width:300px;=22>
                                                <tr>
                                                    <td class=3D=22icon-pad=
ding=22 align=3D=22center=22>
                                                        <a href=3D=22https:=
//emailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODs=
lSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABBRY&amp;_ei_=
=3DEq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=2E&amp;_d=
i_=3Dpjvopco2qqjl652gjqi3fgdcem4gurgdrrcr1a5jas3dj8j3ratg=22 target=3D=22_b=
lank=22 style=3D=22display:block; margin:0; padding:0; line-height:0;=22>
                                                            <img src=3D=22h=
ttps://i1=2Eadis=2Ews/i/boohooamplience/ng_em_icon_facebook=2Epng=22 alt=3D=
=22Facebook=22 width=3D=2242=22 style=3D=22border-width:0; width:100%; max-=
width:42px; height:auto;=22>
                                                        </a>
                                                    </td>
                                                    <td class=3D=22icon-pad=
ding=22 align=3D=22center=22>
                                                        <a href=3D=22https:=
//emailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODs=
lSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABBTY&amp;_ei_=
=3DEq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=2E&amp;_d=
i_=3Dgqqki9h8ec2ugpic6tgkm6qjl8lsq6psvg5qi1o31mfe3uelko70=22 target=3D=22_b=
lank=22 style=3D=22display:block; margin:0; padding:0; line-height:0;=22>
                                                            <img src=3D=22h=
ttps://i1=2Eadis=2Ews/i/boohooamplience/ng_em_icon_twitter=2Epng=22 alt=3D=
=22Twitter=22 width=3D=2242=22 style=3D=22border-width:0; width:100%; max-w=
idth:42px; height:auto;=22>
                                                        </a>
                                                    </td>
                                                    <td class=3D=22icon-pad=
ding=22 align=3D=22center=22>
                                                        <a href=3D=22https:=
//emailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODs=
lSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABBWY&amp;_ei_=
=3DEq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=2E&amp;_d=
i_=3D7n4b00h2pm1maa7knqgr7k1n801ugau1j3719dfn3ls226fpk8f0=22 target=3D=22_b=
lank=22 style=3D=22display:block; margin:0; padding:0; line-height:0;=22>
                                                            <img src=3D=22h=
ttps://i1=2Eadis=2Ews/i/boohooamplience/ng_em_icon_instagram=2Epng=22 alt=
=3D=22Instagram=22 width=3D=2242=22 style=3D=22border-width:0; width:100%; =
max-width:42px; height:auto;=22>
                                                        </a>
                                                    </td>
                                                    <td class=3D=22icon-pad=
ding=22 align=3D=22center=22>
                                                        <a href=3D=22https:=
//emailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODs=
lSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABBAY&amp;_ei_=
=3DEq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=2E&amp;_d=
i_=3Dbls60ivcdfptqnaijarrtqtoueaa90rih9jcafn8f2gphj7n7qig=22 target=3D=22_b=
lank=22 style=3D=22display:block; margin:0; padding:0; line-height:0;=22>
                                                            <img src=3D=22h=
ttps://i1=2Eadis=2Ews/i/boohooamplience/ng_em_icon_snapchat=2Epng=22 alt=3D=
=22Snapchat=22 width=3D=2242=22 style=3D=22border-width:0; width:100%; max-=
width:42px; height:auto;=22>
                                                        </a>
                                                    </td>
                                                </tr>
                                            </table>

                                            <=21--=5Bif (gte mso 9)=7C(IE)=
=5D>
                                                    </td>
                                                </tr>
                                            </table>
                                            <=21=5Bendif=5D-->
                                        </td>
                                    </tr>

                                    <tr>
                                        <td align=3D=22center=22 valign=3D=
=22center=22 style=3D=22padding: 10px 4% 0 4%; font-size:0;=22>
                                            <=21--=5Bif (gte mso 9)=7C(IE)=
=5D>
                                            <table align=3D=22center=22 bor=
der=3D=220=22 cellspacing=3D=220=22 cellpadding=3D=220=22 width=3D=22280=22>
                                                <tr>
                                                    <td align=3D=22center=
=22 valign=3D=22center=22 width=3D=22280=22>
                                            <=21=5Bendif=5D-->

                                            <table align=3D=22center=22 bor=
der=3D=220=22 cellpadding=3D=220=22 cellspacing=3D=220=22 width=3D=22100%=
=22 style=3D=22width: 100%; max-width:280px;=22>
                                                <tr>
                                                    <td align=3D=22center=
=22 width=3D=2250%=22 style=3D=22padding:0;=22>
                                                        <a href=3D=22https:=
//emailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODs=
lSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABBCY&amp;_ei_=
=3DEq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=2E&amp;_d=
i_=3D5on5d1nrsminfnbcqejmcvjbbp4bdav6bpghmv3h3guc2n4vq8fg=22 target=3D=22_b=
lank=22 border=3D=220=22 style=3D=22display:block; outline:none; margin:0; =
padding:0 5%; line-height:0;=22>
                                                            <img src=3D=22h=
ttps://i1=2Eadis=2Ews/i/boohooamplience/ng_em_icon_app_store=2Epng=22 alt=
=3D=22App Store=22 width=3D=22100=22 style=3D=22display:block; border-width=
:0; width:100%; max-width:120px; height:auto;=22>
                                                        </a>
                                                    </td>
                                                    <td align=3D=22center=
=22 width=3D=2250%=22 style=3D=22padding:0;=22>
                                                        <a href=3D=22https:=
//emailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODs=
lSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABCRY&amp;_ei_=
=3DEq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=2E&amp;_d=
i_=3Dir3j6edi996e8gar8faaq4gbg67d50p670n4jki495l1a2ii4m5g=22 target=3D=22_b=
lank=22 border=3D=220=22 style=3D=22display:block; outline:none; margin:0; =
padding:0 5%; line-height:0;=22>
                                                            <img src=3D=22h=
ttps://i1=2Eadis=2Ews/i/boohooamplience/ng_em_icon_google_play=2Epng=22 alt=
=3D=22Google Play=22 width=3D=22100=22 style=3D=22display:block; border-wid=
th:0; width:100%; max-width:120px; height:auto;=22>
                                                        </a>
                                                    </td>
                                                </tr>
                                            </table>
                                           =20
                                            <=21--=5Bif (gte mso 9)=7C(IE)=
=5D>
                                                    </td>
                                                </tr>
                                            </table>
                                            <=21=5Bendif=5D-->
                                        </td>
                                    </tr>
                                </table>
                            </div>
                            <=21-- ? END: APP ICONS -->

                            <=21-- =23 UNSUBSCRIBE COPY -->
                            <div style=3D=22display:block;=22>
                                <table class=3D=22footer=22 align=3D=22cent=
er=22 border=3D=220=22 cellpadding=3D=220=22 cellspacing=3D=220=22 width=3D=
=22100%=22 style=3D=22max-width:700px;=22>
                                    <tr>
                                        <td align=3D=22center=22 style=3D=
=22padding:3% 0 0 0; text-align:center;=22>
                                            <table class=3D=22footer=22 ali=
gn=3D=22center=22 border=3D=220=22 cellpadding=3D=220=22 cellspacing=3D=220=
=22 width=3D=22100%=22 style=3D=22max-width:700px;=22>
                                                <tr>
                                                    <td align=3D=22center=
=22 style=3D=22padding:15px 0 0 0; text-align:center;=22>
                                                        <p class=3D=22copy_=
legal=22 style=3D=22font-family: 'NG-Grotesque', Helvetica, Arial, sans-ser=
if; color: =23999999;=22>Score up to 80% off everything + an extra 5% off=
=2E Use code TAKE5=2E Valid 13=2E12=2E20 for UK only=2E
                                                        <br>
                                                        <br>
                                                            *To qualify for=
 a free gift you must spend =C2=A345, choose 1 product from the free gift c=
ategory and enter code XMASTREAT at checkout=2E Valid 13=2E12=2E20 for UK o=
nly=2E
                                                        </p>
                                                    </td>
                                                </tr>
                                                <tr>
                                                    <td align=3D=22center=
=22 style=3D=22padding:15px 0 0 0; text-align:center;=22>
                                                        <p class=3D=22copy_=
legal=22 style=3D=22font-family:'NG-Grotesque', Helvetica, Arial, sans-seri=
f; font-weight:300; color:=23999999;=22>
                                                            <a href=3D=22ht=
tps://emailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvM=
bODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABCTY&amp;_=
ei_=3DEq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=2E&amp=
;_di_=3Dmojde2d19rvv4ii0v5rlf978vu2h9lo0mlch4ee15hd9h0a4vek0=22 style=3D=22=
color:=23999999; text-decoration:none;=22>Contact Us</a>
                                                            <span style=3D=
=22color: =23999999;=22>&nbsp;&bull;&nbsp;</span>
                                                            <a href=3D=22ht=
tps://emailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvM=
bODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABCWY&amp;_=
ei_=3DEq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=2E&amp=
;_di_=3Dp8g0op2prl6o3tt4s0dkjh052kit0c1h5lq347rik56ebn7eqrj0=22 style=3D=22=
color:=23999999; text-decoration:none;=22>Privacy Notice</a>
                                                            <span style=3D=
=22color: =23999999;=22>&nbsp;&bull;&nbsp;</span>
                                                            <a href=3D=22ht=
tps://emailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvM=
bODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABCCY&amp;_=
ei_=3DEiwPQ42l-mBFJGD0ZPxQdvmMwp2_P3883g3SgIn45kl11N0wWt6_E46BLadSi_dmqTgkg=
CDvFrP6oFwUTMtZD-IHKqr1E8-mdpnOblAZKvJVGkWMs1yh6iemhPRqbzzMTgaV-o8wDBF7ZoMX=
wgi79Iukt4ppnBRaUNE=2E&amp;_di_=3Da4mis16va320du0667rrkl5rc8456kop00053ja2l=
urr7ggecjkg=22 style=3D=22color:=23999999; text-decoration:none;=22>Unsubsc=
ribe</a>
                                                            <span style=3D=
=22color: =23999999;=22>&nbsp;&bull;&nbsp;</span>
                                                            <a href=3D=22ht=
tps://emailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvM=
bODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABCAY&amp;_=
ei_=3DEq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=2E&amp=
;_di_=3Deg1opg7loncukf8494r3vgeimqer58t35mc5n26dkptf10nhrlt0=22 style=3D=22=
color:=23999999; text-decoration:none;=22>View in browser</a>
                                                        </p>
                                                    </td>
                                                </tr>
                                                <tr>
                                                    <td align=3D=22center=
=22 style=3D=22padding:15px 0 0 0; text-align:center;=22>
                                                        <p class=3D=22copy_=
legal=22 style=3D=22font-family:'NG-Grotesque', Helvetica, Arial, sans-seri=
f; color:=23999999;=22>This email was sent by: Nasty Gal A Company register=
ed in California<br/> =7BNO=2E 10487954 =7D Registered office: 2135 Bay St,=
 Los Angeles, CA 90021</p>
                                                    </td>
                                                </tr>
                                                <tr>
                                                    <td align=3D=22center=
=22 style=3D=22padding:20px 0 20px 0; text-align:center;=22>
                                                        <a href=3D=22https:=
//emailnastygal=2Ecom/pub/cc?_ri_=3DX0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODs=
lSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABWRY&amp;_ei_=
=3DEq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM=2E&amp;_d=
i_=3Dm0oj3n5jg9cvb3rkosk0qekegvb7ej69ur7c2tualdrnareu4kng=22 target=3D=22_b=
lank=22>
                                                            <img src=3D=22h=
ttps://i1=2Eadis=2Ews/i/boohooamplience/ng_em_icon=2Epng=22 alt=3D=22Nasty =
Gal=22 width=3D=2230=22 border=3D=220=22 style=3D=22border:0;=22 />
                                                        </a>
                                                    </td>
                                                </tr>
                                            </table>
                                        </td>
                                    </tr>
                                </table>
                            </div>
                            <=21-- ? END: UNSUBSCRIBE COPY -->

                        </td>
                    </tr>
                </table>
                <=21--=5Bif (gte mso 9)=7C(IE)=5D>
                        </td>
                    </tr>
                </table>
                <=21=5Bendif=5D-->
            </td>
        </tr>
        <=21-- * END: FOOTER -->

    </table><table cellpadding=3D=220=22 cellspacing=3D=220=22 style=3D=22b=
order: 0px; padding: 0px; margin: 0px; position: absolute; display: none; f=
loat: left=22>
<tr>
<td height=3D=221=22 style=3D=22font-size: 1px; line-height: 1px; padding: =
0px;=22>
<br><img src=3D=22https://emailnastygal=2Ecom/pub/as?_ri_=3DX0Gzc2X%3DAQpgl=
LjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXHk=
MX%3Dw&_ei_=3DEolaGGF4SNMvxFF7KucKuWNjwOLyncDiCbA_Akyj-vMWiVKaEzA=2E=22></i=
mg>
</td>
</tr>
</table>

</body>

</html>`;

const sampleHtml = `<!DOCTYPE html>\n<html lang="en" xmlns="http://www.w3.org/1999/xhtml" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office">\n\n<head>\n    <meta charset="utf-8">\n    <meta name="viewport" content="width=device-width, initial-scale=1">\n    <meta http-equiv="X-UA-Compatible" content="IE=edge">\n    <meta name="x-apple-disable-message-reformatting">\n\n    <title>Nasty Girl</title>\n\n    <!-- // Ooutlook DPI Scaling Fix -->\n    <!--[if gte mso 9]>\n    <xml>\n        <o:OfficeDocumentSettings>\n            <o:AllowPNG/>\n            <o:PixelsPerInch>96</o:PixelsPerInch>\n        </o:OfficeDocumentSettings>\n    </xml>\n    <![endif]-->\n\n\n    <style media="all" type="text/css">\n        /*--- Fonts ---*/\n        @font-face{\n            font-family:\'Open Sans\';\n            font-style:normal;\n            font-weight:400;\n            src:local(\'Open Sans\'), local(\'OpenSans\'), url(\'http://fonts.gstatic.com/s/opensans/v10/cJZKeOuBrn4kERxqtaUH3bO3LdcAZYWl9Si6vvxL-qU.woff\') format(\'woff\');\n        }\n        @font-face {\n            font-family: \'NG-Grotesque\';\n            src: url(\'https://static.cdn.responsys.net/i5/responsysimages/content/boohoo/NGGrotesque1.eot\');\n            src: url(\'https://static.cdn.responsys.net/i5/responsysimages/content/boohoo/NGGrotesque1.eot?iefix\') format(\'embedded-opentype\'),\n                url(\'https://www.boohoo.com/on/demandware.static/-/Library-Sites-boohoo-content-global/default/dw79a56c45/landing-pages/global-elements/fonts/NGGrotesque/ng-grotesque-1.otf\') format(\'opentype\'),\n                url(\'https://static.cdn.responsys.net/i5/responsysimages/content/boohoo/NGGrotesque1.woff\') format(\'woff\');\n            font-weight: 300;\n            font-style: normal;\n            mso-font-alt: \'Arial\';\n        }\n        @font-face {\n            font-family: \'NG-Grotesque\';\n            src: url(\'https://www.boohoo.com/on/demandware.static/-/Library-Sites-boohoo-content-global/default/dw79a56c45/landing-pages/global-elements/fonts/NGGrotesque/ng-grotesque-2.otf\') format(\'opentype\');\n            font-weight: normal;\n            font-style: normal;\n            mso-font-alt: \'Arial\';\n        }\n        @font-face {\n            font-family: \'NG-Grotesque\';\n            src: url(\'https://static.cdn.responsys.net/i5/responsysimages/content/boohoo/NGGrotesque3.eot\');\n            src: url(\'https://static.cdn.responsys.net/i5/responsysimages/content/boohoo/NGGrotesque3.eot?iefix\') format(\'embedded-opentype\'),\n                url(\'https://www.boohoo.com/on/demandware.static/-/Library-Sites-boohoo-content-global/default/dw79a56c45/landing-pages/global-elements/fonts/NGGrotesque/ng-grotesque-3.otf\') format(\'opentype\'),\n                url(\'https://static.cdn.responsys.net/i5/responsysimages/content/boohoo/NGGrotesque3.woff\') format(\'woff\');\n            font-weight: bold;\n            font-style: normal;\n            mso-font-alt: \'Arial\';\n        }\n    </style>\n\n    <style type="text/css">\n    html, body {\n\t\t\tmargin: 0 auto !important;\n\t\t\tpadding: 0 !important;\n\t\t\theight: 100% !important;\n\t\t\twidth: 100% !important;\n            font-family: Arial, sans-serif;\n            background-color: #EEEEEE;\n        }\n        \n        .bg_color { background-color: #EEEEEE; }\n        .bg_white { background-color: #FFFFFF; }\n\n        .wrapper { table-layout: fixed !important; }\n        .tb_images { table-layout: auto !important; }\n\n        .mobile-hide { display: none !important; }\n\n        .padding { padding: 10px 5% 10px 5%; }\n        .icon-padding { padding: 10px 8px 10px 8px; text-align: center; }\n        .no-padding { padding: 0 !important; }\n        .section-padding { padding: 50px 15px 50px 15px !important; }\n\n        .copy_top_text { font-family: \'NG-Grotesque\', Helvetica, Arial, sans-serif; font-size: 11px; line-height: 100%; font-weight: 300; color: #999999; }\n        .copy_top_text a { color:#999999; text-decoration:none; }\n        .copy_top_text a:hover { color:#999999; text-decoration:underline; }\n\n        .logo { display: inline-block; font-size: 0; line-height: 0px; margin: 0; padding: 0; }\n        .logo img { margin: 0 auto !important; max-width: 200px; margin: 0; padding: 0; }\n\n        .header_button { font-family: \'NG-Grotesque\', Helvetica, Arial, sans-serif; font-size: 16px; font-weight: 300; color: #000000; margin: 0; padding: 0; }\n        .header_button a { color: #000000; text-decoration:none; }\n        .header_button a:hover { color: #000000; text-decoration:underline; }\n\n        .copy_heading { font-family: \'NG-Grotesque\', Helvetica, Arial, sans-serif; font-size: 32px; font-weight: 300; color: #000000; margin: 0; padding: 0; }\n        .copy_heading a { color:#000000; text-decoration:none; }\n        .copy_heading a:hover { color:#000000; text-decoration:underline; }\n\n        .copy_body{ font-family: \'NG-Grotesque\', Helvetica, Arial, sans-serif; font-size: 18px; font-weight: 300; color: #000000; margin: 0; padding: 0; }\n        .copy_body a { color:#000000; text-decoration:none; }\n        .copy_body a:hover { color:#000000; text-decoration:underline; }\n\n        .cta_button { font-family: \'NG-Grotesque\', Helvetica, Arial, sans-serif; font-size: 16px; font-weight: 300; color: #000000; margin: 0; padding: 0; }\n        .cta_button a { color:#000000; text-decoration:none; }\n        .cta_button a:hover { color:#000000; text-decoration:none; }\n\n        .copy_legal { font-family: \'NG-Grotesque\', Helvetica, Arial, sans-serif; font-size:13px; line-height:18px; font-weight: 300; color:#666666; margin: 0; padding: 0; }\n        .copy_legal a { color:#666666; text-decoration:none; }\n        .copy_legal a:hover { color:#666666; text-decoration:underline; }\n\n\n        /* MOBILE STYLES */\n        @media screen and (max-width: 700px) {\n            .img-max { max-width: 100% !important; width: 100% !important; height: auto !important; }\n            .img-padded { max-width: 96% !important; width: 96% !important; height: auto !important; }\n        }\n\n        @media screen and (max-width: 480px) {\n            .wrapper { width: 100% !important; max-width: 100% !important; }\n            .responsive-table { width: 100% !important; }\n\n            .copy_top_text { font-size: 10px; }\n            .logo img { margin: 0 auto !important; max-width: 68%; }\n            .header_buttons { width: 90% !important; max-width: 90% !important; }\n            .header_button { font-size: 12px; }\n\n            .copy_heading { font-size: 24px; }\n            .copy_body { font-size: 14px; }\n            .cta_button { font-size: 13px; }\n\n            .icon-padding { padding: 10px 5% 10px 5%; text-align: center; }\n            .social_icons { width: 70% !important; max-width: 70% !important; }\n            .app_icons { width: 60% !important; max-width: 60% !important; }\n            .copy_legal { font-size: 11px; line-height: 15px; }\n        }\n    </style>\n\n    <style type="text/css">\n        /* CSS Resets */\n        #MessageViewBody, #MessageWebViewDiv{ width: 100% !important; }\n        * { -ms-text-size-adjust: 100%; -webkit-text-size-adjust: 100%; }\n        *[x-apple-data-detectors], .unstyle-auto-detected-links *, .aBn { \n            border-bottom: 0 !important; \n            cursor: default !important; \n            color: inherit !important; \n            text-decoration: none !important; \n            font-size: inherit !important; \n            font-family: inherit !important; \n            font-weight: inherit !important; \n            line-height: inherit !important; \n        }\n\n        div[style*="margin: 16px 0"] { margin: 0 auto !important; font-size:100% !important; }\n        body, table, td, a {-webkit-text-size-adjust: 100%; -ms-text-size-adjust: 100%;}\n        table, td { mso-table-lspace: 0pt !important; mso-table-rspace: 0pt !important; }\n        table { border-spacing: 0 !important; border-collapse: collapse !important; margin: 0 auto !important; }\n        img { -ms-interpolation-mode:bicubic; }\n        a { text-decoration: none; }\n        .a6S { display: none !important; opacity: 0.01 !important; }\n        .im { color: inherit !important; }\n        img.g-img + div { display: none !important; }\n        \n        /* RESET STYLES */\n        img {border: 0; height: auto; line-height: 100%; outline: none; text-decoration: none;}\n        table {border-collapse: collapse !important;}\n        body {height: 100% !important; margin: 0 !important; padding: 0 !important; width: 100% !important; }\n\n        /* iPhone Fixes */\n        @media only screen and (min-device-width: 320px) and (max-device-width: 374px) {u ~ div .em_container { min-width: 320px !important; }}\n        @media only screen and (min-device-width: 375px) and (max-device-width: 413px) {u ~ div .em_container { min-width: 375px !important; }}\n        @media only screen and (min-device-width: 414px) {u ~ div .em_container { min-width: 414px !important; }}\n        @media screen and (max-width: 480px) {u + .body-wrap .full-wrap { width:100% !important; width:100vw !important; }}\n    </style>\n\n    <!--[if (gte mso 9)|(IE)]>\n\t<style type="text/css">\n\t\ttable {border-collapse: collapse !important;}\n\t\th1, h2, h3, h4, h5, h6, p, a {font-family: Arial, sans-serif !important;}\n\t</style>\n\t<![endif]-->\n</head>\n\n<body class="bg_color" style="background-color:#EEEEEE; margin:0 !important; padding:0 !important;">\n    <!-- // HIDDEN TEXT -->\n    <div style="display:none; font-size:1px; line-height:1px; font-family:Arial, sans-serif; max-height:0px; max-width:0px; opacity:0; overflow:hidden;">\n        Handpicked just for you. \n        &zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;&zwnj;&nbsp;\n    </div>\n\n    <!-- ! WRAPPER -->\n    <table border="0" cellpadding="0" cellspacing="0" width="100%" class="wrapper">\n\n        <!-- @ HEADER TEXT -->\n        <tr>\n            <td align="center">\n                <!--[if (gte mso 9)|(IE)]>\n                <table align="center" border="0" cellspacing="0" cellpadding="0" width="700">\n                    <tr>\n                        <td align="center" valign="top" width="700">\n                <![endif]-->\n\n                <table class="wrapper" border="0" cellpadding="0" cellspacing="0" width="100%" style="width:100%; max-width:700px;">\n                    <tr>\n                        <td class="copy_top_text" align="center" style="padding:10px 0 8px 0; text-align:center;">\n                            <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABWRY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=m0oj3n5jg9cvb3rkosk0qekegvb7ej69ur7c2tualdrnareu4kng" style="font-family:\'NG-Grotesque\', Helvetica, Arial, sans-serif; font-weight:300; color:#999999; text-decoration:none;">Handpicked just for you. </a>\n                        </td>\n                    </tr>\n                </table>\n\n                <!--[if (gte mso 9)|(IE)]>\n                        </td>\n                    </tr>\n                </table>\n                <![endif]-->\n            </td>\n        </tr>\n        <!-- * HEADER TEXT -->\n\n        <!-- @ HEADER -->\n        <tr>\n            <td align="center" style="padding:0" class="no-padding">\n                <!--[if (gte mso 9)|(IE)]>\n                <table align="center" border="0" cellspacing="0" cellpadding="0" width="700">\n                    <tr>\n                        <td align="center" valign="top" width="700">\n                <![endif]-->\n\n                <table class="responsive-table  bg_white" bgcolor="#FFFFFF" border="0" cellpadding="0" cellspacing="0" width="100%" style="width:100%; max-width:700px; background-color: #FFFFFF;">\n                    <!-- # LOGO -->\n                    <tr>\n                        <td align="center">\n                            <!--[if (gte mso 9)|(IE)]>\n                            <table align="center" border="0" cellspacing="0" cellpadding="0" width="200">\n                                <tr>\n                                    <td align="center" valign="top" width="200">\n                            <![endif]-->\n\n                            <table class="wrapper" align="center" width="100%" border="0" cellspacing="0" cellpadding="0" style="width:100%; max-width:700px;">\n                                <tr>\n                                    <td align="center" style="padding:3% 0 1% 0;">\n                                        <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABWRY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=m0oj3n5jg9cvb3rkosk0qekegvb7ej69ur7c2tualdrnareu4kng" target="_blank" class="logo" style="display:inline-block; font-size:0px; line-height:0px;">\n                                            <img src="https://i1.adis.ws/i/boohooamplience/ng_em_logo.png" alt="Nasty Gal" border="0" width="200" style="width:100%; max-width:200px; display:inline-block; border:0; font-size:0px; line-height:0px;">\n                                        </a>\n                                    </td>\n                                </tr>\n                            </table>\n                            \n                            <!--[if (gte mso 9)|(IE)]>\n                                    </td>\n                                </tr>\n                            </table>\n                            <![endif]-->\n                        </td>\n                    </tr>\n                    <!-- ? END: LOGO -->\n\n                    <!-- # HEADER BUTTONS -->\n                    <tr>\n                        <td align="center">\n                            <!--[if (gte mso 9)|(IE)]>\n                            <table align="center" border="0" cellspacing="0" cellpadding="0" width="400">\n                                <tr>\n                                    <td align="center" valign="top" width="400">\n                            <![endif]-->\n\n                            <div style="display:block;">\n                                <table class="wrapper" align="center" border="0" cellpadding="0" cellspacing="0" width="100%" style="width:100%; max-width:700px;">\n                                    <tr>\n                                        <td align="center" valign="top" style="padding:2% 0 3% 0; font-size:0;">\n                                            <div class="header_buttons" style="display:inline-block; margin: 0; width:100%; max-width:400px; vertical-align:top;">\n                                                <table class="tb_images" align="center" border="0" cellpadding="0" cellspacing="0" width="100%" style="width:100%; table-layout:auto;">\n                                                    <tr>\n                                                        <td align="center">\n                                                            <div style="border-right:solid 1px #444444; font-size:16px; font-weight:300;">\n                                                                <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABWTY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=vgvj54lpd73i1f5m9ak08ouopkkrti8c04uh1aidtva7uq2qknt0" target="_blank" class="header_button" style="display:inline-block; margin:0; padding:0; color:#000000; box-sizing:border-box; cursor:pointer; text-align:center; text-decoration:none;">Shop New</a>\n                                                            </div>\n                                                        </td>\n                                                        <td align="center">\n                                                            <div style="border-right:solid 1px #444444; font-size:16px; font-weight:300;">\n                                                                <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABWWY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=simruh5rdkjb6msanr54cqdb0vp181bgea5nhmo2lvi8lrrd5jtg" target="_blank" class="header_button" style="display:inline-block; margin:0; padding:0; color:#000000; box-sizing:border-box; cursor:pointer; text-align:center; text-decoration:none;">Shop Dresses</a>\n                                                            </div>\n                                                        </td>\n                                                        <td align="center">\n                                                            <div style="font-size:16px; font-weight:300;">\n                                                                <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABWAY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=3k42p9u5891tup5tm6i7bqegaiks1okr2v136jp00feotfedbv10" target="_blank" class="header_button" style="display:inline-block; margin:0; padding:0; color:#000000; box-sizing:border-box; cursor:pointer; text-align:center; text-decoration:none;">Shop Shoes</a>\n                                                            </div>\n                                                        </td>\n                                                    </tr>\n                                                </table>\n                                            </div>\n                                        </td>\n                                    </tr>\n                                </table>\n                            </div>\n\n                            <!--[if (gte mso 9)|(IE)]>\n                                    </td>\n                                </tr>\n                            </table>\n                            <![endif]-->\n                        </td>\n                    </tr>\n                    <!-- ? END: FOOTER BUTTONS --> \n\n                </table>\n\n                <!--[if (gte mso 9)|(IE)]>\n                        </td>\n                    </tr>\n                </table>\n                <![endif]-->\n            </td>\n        </tr>\n        <!-- * END: HEADER -->\n\n        <!-- @ EMAIL CONTENT -->\n        <tr>\n            <td align="center" style="padding:0" class="no-padding">\n                <!--[if (gte mso 9)|(IE)]>\n                <table align="center" border="0" cellspacing="0" cellpadding="0" width="700">\n                    <tr>\n                        <td align="center" valign="top" width="700">\n                <![endif]-->\n\n                <table class="responsive-table  bg_white" bgcolor="#FFFFFF" border="0" cellpadding="0" cellspacing="0" width="100%" style="width:100%; max-width:700px; background-color: #FFFFFF;">\n                    <!-- # CONTENT IMAGES -->\n                    <tr>\n                        <td align="center" style="padding:0" class="no-padding">\n\n\n\n\n\n\t\t\t\t\t\t\t<!-- # HERO IMAGE -->\n                            <table class="wrapper" width="100%" border="0" cellspacing="0" cellpadding="0">\n                                <tr>\n                                    <td class="no-padding" align="center">\n                                        <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABWCY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=cqi7s7mk1n73plv2pcknjm3gh3fquh2p0euphe2hh7vma8v8dhu0" target="_blank">\n                                            <img src="https://static.cdn.responsys.net/i5/responsysimages/nastygal/contentlibrary/2020/12december/gbp/201213_gbp_wintershop_vs/uk/UK_Email_Banner.jpg" alt="Nasty Gal" border="0" width="700" style="width:100%; max-width:700px; display:block; font-size:0;" class="img-max">\n                                        </a>\n                                    </td>\n                                </tr>\n                            </table>\n                            <!-- ? END: HERO IMAGE -->\n\t\t\t\t\t\t\t\n\t\t\t\t\t\t\t<!-- # HERO IMAGE -->\n                            <table class="wrapper" width="100%" border="0" cellspacing="0" cellpadding="0">\n                                <tr>\n                                    <td class="no-padding" align="center"><a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABYRY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=bracm8s9seoo0bvhgpoa7unbsjjar6i14an5esrmphpvq2qi76d0" target="_blank"><img alt="Nasty Gal" border="0" class="img-max" src="https://static.cdn.responsys.net/i5/responsysimages/content/nastygal/WINTERSHOPHERO.gif?imbypass=true" style="width:100%; max-width:700px; display:block; font-size:0;" width="700" /> </a></td>\n                                </tr>\n                            </table>\n                            <!-- ? END: HERO IMAGE -->\n\t\t\t\t\t\t\t\n\t\t\t\t\t\t\t  <!-- # 2 IMAGES -->\n                            <table class="wrapper" width="100%" border="0" cellspacing="0" cellpadding="0" style="width:100%; max-width:700px;">\n                                <tr>\n                                    <td align="center" style="padding:0;">\n\n                                        <table class="tb_images" width="100%" border="0" cellspacing="0" cellpadding="0" style="width:100%; max-width:700px;">\n                                            <tr valign="top">\n                                                <td class="no-padding" align="center">\n                                                    <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABYTY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=ooam4b0t3ipvmi10jnevk95mdghbqndt3pfgiei76khl6rdtsr8g" target="_blank">\n                                                        <img src="https://static.cdn.responsys.net/i5/responsysimages/nastygal/contentlibrary/2020/12december/gbp/201213_gbp_wintershop_vs/uk/Winter_Shop2_02.jpg" alt="Nasty Gal" border="0" width="350" style="width:100%; max-width:350px; display:block; font-size:0;" class="no-padding">\n                                                    </a>\n                                                </td>\n                                                <td class="no-padding" align="center">\n                                                    <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABYWY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=b3ngb0htt8hm3t1m2js8vfkaljaoq1qle8hqephkacjign9ll4p0" target="_blank">\n                                                        <img src="https://static.cdn.responsys.net/i5/responsysimages/nastygal/contentlibrary/2020/12december/gbp/201213_gbp_wintershop_vs/uk/Winter_Shop2_03.jpg" alt="Nasty Gal" border="0" width="350" style="width:100%; max-width:350px; display:block; font-size:0;" class="no-padding">\n                                                    </a>\n                                                </td>\n                                            </tr>\n                                        </table>\n\n                                    </td>\n                                </tr>\n                            </table>\n                            <!-- ? END: 2 IMAGES -->\n\t\t\t\t\t\t\t\n\t\t\t\t\t\t\t  <!-- # 2 IMAGES -->\n                            <table class="wrapper" width="100%" border="0" cellspacing="0" cellpadding="0" style="width:100%; max-width:700px;">\n                                <tr>\n                                    <td align="center" style="padding:0;">\n\n                                        <table class="tb_images" width="100%" border="0" cellspacing="0" cellpadding="0" style="width:100%; max-width:700px;">\n                                            <tr valign="top">\n                                                <td class="no-padding" align="center">\n                                                    <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABYAY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=3tgl7vgau5s27qhg53m79epiusdh6r62eaolbgptau01cn96nntg" target="_blank">\n                                                        <img src="https://static.cdn.responsys.net/i5/responsysimages/nastygal/contentlibrary/2020/12december/gbp/201213_gbp_wintershop_vs/uk/Winter_Shop2_04.jpg" alt="Nasty Gal" border="0" width="350" style="width:100%; max-width:350px; display:block; font-size:0;" class="no-padding">\n                                                    </a>\n                                                </td>\n                                                <td class="no-padding" align="center">\n                                                    <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABYCY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=5og5cd26u4pm2m89fnekc98327jqqo3arq6od8g8vfro93d59mng" target="_blank">\n                                                        <img src="https://static.cdn.responsys.net/i5/responsysimages/nastygal/contentlibrary/2020/12december/gbp/201213_gbp_wintershop_vs/uk/Winter_Shop2_05.jpg" alt="Nasty Gal" border="0" width="350" style="width:100%; max-width:350px; display:block; font-size:0;" class="no-padding">\n                                                    </a>\n                                                </td>\n                                            </tr>\n                                        </table>\n\n                                    </td>\n                                </tr>\n                            </table>\n                            <!-- ? END: 2 IMAGES -->\n\n\n\t\t\t\t\t\t\t<!-- # HERO IMAGE -->\n                            <table class="wrapper" width="100%" border="0" cellspacing="0" cellpadding="0">\n                                <tr>\n                                    <td class="no-padding" align="center"><a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABARY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=6nn5bnaa8a2b4ld3tftuv9mqehv1ehkhbbl711c2mhusmb1n3t9g" target="_blank"><img alt="Nasty Gal" border="0" class="img-max" src="https://static.cdn.responsys.net/i5/responsysimages/content/nastygal/WINTERSHOP2.gif?imbypass=true" style="width:100%; max-width:700px; display:block; font-size:0;" width="700" /> </a></td>\n                                </tr>\n                            </table>\n                            <!-- ? END: HERO IMAGE -->\n\n\n\t\t\t\t\t\t\t  <!-- # 2 IMAGES -->\n                            <table class="wrapper" width="100%" border="0" cellspacing="0" cellpadding="0" style="width:100%; max-width:700px;">\n                                <tr>\n                                    <td align="center" style="padding:0;">\n\n                                        <table class="tb_images" width="100%" border="0" cellspacing="0" cellpadding="0" style="width:100%; max-width:700px;">\n                                            <tr valign="top">\n                                                <td class="no-padding" align="center">\n                                                    <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABATY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=3ct5booibqin3htuquqpj3jh3t7ata7apod1mtb8s5p1g155svqg" target="_blank">\n                                                        <img src="https://static.cdn.responsys.net/i5/responsysimages/nastygal/contentlibrary/2020/12december/gbp/201213_gbp_wintershop_vs/uk/New_Winter_Shop2_07.jpg" alt="Nasty Gal" border="0" width="350" style="width:100%; max-width:350px; display:block; font-size:0;" class="no-padding">\n                                                    </a>\n                                                </td>\n                                                <td class="no-padding" align="center">\n                                                    <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABAWY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=pnm8f70h2du6o0oriknj4mubad459a16fa86kb3qqhe7ch737c20" target="_blank">\n                                                        <img src="https://static.cdn.responsys.net/i5/responsysimages/nastygal/contentlibrary/2020/12december/gbp/201213_gbp_wintershop_vs/uk/New_Winter_Shop2_08.jpg" alt="Nasty Gal" border="0" width="350" style="width:100%; max-width:350px; display:block; font-size:0;" class="no-padding">\n                                                    </a>\n                                                </td>\n                                            </tr>\n                                        </table>\n\n                                    </td>\n                                </tr>\n                            </table>\n                            <!-- ? END: 2 IMAGES -->\n\t\t\t\t\t\t\t\n\t\t\t\t\t\t\t<!-- # HERO IMAGE -->\n                            <table class="wrapper" width="100%" border="0" cellspacing="0" cellpadding="0">\n                                <tr>\n                                    <td class="no-padding" align="center">\n                                        <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABAAY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=haqcb03ujkp30onoj5q5kujvnvmsf894khj93vdvnnp95vhm7adg" target="_blank">\n                                            <img src="https://static.cdn.responsys.net/i5/responsysimages/nastygal/contentlibrary/2020/12december/gbp/201213_gbp_wintershop_vs/uk/Winter_Shop2_08.jpg" alt="Nasty Gal" border="0" width="700" style="width:100%; max-width:700px; display:block; font-size:0;" class="img-max">\n                                        </a>\n                                    </td>\n                                </tr>\n                            </table>\n                            <!-- ? END: HERO IMAGE -->\n\t\t\t\t\t\t\t\n\t\t\t\t\t\t\t<!-- # HERO IMAGE -->\n                            <table class="wrapper" width="100%" border="0" cellspacing="0" cellpadding="0">\n                                <tr>\n                                    <td class="no-padding" align="center"><a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABACY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=cslqk6dejuj4bhfjvvgm3mu276qlbg9na78kfitvon88tekd21g0" target="_blank"><img alt="Nasty Gal" border="0" class="img-max" src="https://static.cdn.responsys.net/i5/responsysimages/nastygal/contentlibrary/2020/12december/gbp/201213_gbp_wintershop_vs/uk/New_UK_Promo_Postcard_02.gif" style="width:100%; max-width:700px; display:block; font-size:0;" width="700" /> </a></td>\n                                </tr>\n                            </table>\n                            <!-- ? END: HERO IMAGE -->\n\t\t\t\t\t\t\t\n\n                        </td>\n                    </tr>\n                    <!-- ? END: CONTENT IMAGES -->\n                </table>\n\n                <!--[if (gte mso 9)|(IE)]>\n                        </td>\n                    </tr>\n                </table>\n                <![endif]-->\n            </td>\n        </tr>\n        <!-- * END: EMAIL CONTENT -->\n\n\n        <!-- @ FOOTER -->\n        <tr>\n            <td align="center" style="padding:0" class="no-padding">\n                <!--[if (gte mso 9)|(IE)]>\n                <table align="center" border="0" cellspacing="0" cellpadding="0" width="700">\n                    <tr>\n                        <td align="center" valign="top" width="700">\n                <![endif]-->\n\n                <!-- # HR BAR -->\n                <table class="responsive-table  bg_white" border="0" cellpadding="0" cellspacing="0" width="100%" style="width:100%; max-width:700px; background-color: #FFFFFF;">\n                    <tr>\n                        <td align="center" style="margin:0; padding: 50px 5% 0 5%; text-align:center;">\n                            <table align="center" cellspacing="0" cellpadding="0" border="0" width="96%" style="margin:0; padding:0; border-spacing:0; width:96%; max-width:700px; text-align:center;">\n                                <tr>\n                                    <td width="96%" height="2" style="background-color: #CCCCCC; border-collapse:collapse; mso-table-lspace:0pt; mso-table-rspace:0pt; mso-line-height-rule:exactly; line-height:2px; width:96%; max-width:700px;">\n                                        <!--[if gte mso 15]>&nbsp;<![endif]-->\n                                    </td>\n                                </tr>\n                            </table>\n                        </td>\n                    </tr>\n                </table>\n                <!-- ? END: HR -->\n\n                <table bgcolor="#FFFFFF" border="0" cellpadding="0" cellspacing="0" width="100%" style="width:100%; max-width:700px;" class="responsive-table">\n                    <tr>\n                        <td align="center" height="100%" valign="top" width="100%">\n\n                            <!-- # APP ICONS -->\n                            <div style="display:block;">\n                                <table align="center" border="0" cellpadding="0" cellspacing="0" width="100%" style="width:100%; max-width:700px;">\n                                    <tr>\n                                        <td align="center" valign="center" style="padding: 30px 1% 0 1%; font-size:0;">\n                                            <!--[if (gte mso 9)|(IE)]>\n                                            <table align="center" border="0" cellspacing="0" cellpadding="0" width="300">\n                                                <tr>\n                                                    <td align="center" valign="center" width="300">\n                                            <![endif]-->\n\n                                            <table align="center" border="0" cellpadding="0" cellspacing="0" width="100%" style="width: 100%; max-width:300px;">\n                                                <tr>\n                                                    <td class="icon-padding" align="center">\n                                                        <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABBRY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=pjvopco2qqjl652gjqi3fgdcem4gurgdrrcr1a5jas3dj8j3ratg" target="_blank" style="display:block; margin:0; padding:0; line-height:0;">\n                                                            <img src="https://i1.adis.ws/i/boohooamplience/ng_em_icon_facebook.png" alt="Facebook" width="42" style="border-width:0; width:100%; max-width:42px; height:auto;">\n                                                        </a>\n                                                    </td>\n                                                    <td class="icon-padding" align="center">\n                                                        <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABBTY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=gqqki9h8ec2ugpic6tgkm6qjl8lsq6psvg5qi1o31mfe3uelko70" target="_blank" style="display:block; margin:0; padding:0; line-height:0;">\n                                                            <img src="https://i1.adis.ws/i/boohooamplience/ng_em_icon_twitter.png" alt="Twitter" width="42" style="border-width:0; width:100%; max-width:42px; height:auto;">\n                                                        </a>\n                                                    </td>\n                                                    <td class="icon-padding" align="center">\n                                                        <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABBWY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=7n4b00h2pm1maa7knqgr7k1n801ugau1j3719dfn3ls226fpk8f0" target="_blank" style="display:block; margin:0; padding:0; line-height:0;">\n                                                            <img src="https://i1.adis.ws/i/boohooamplience/ng_em_icon_instagram.png" alt="Instagram" width="42" style="border-width:0; width:100%; max-width:42px; height:auto;">\n                                                        </a>\n                                                    </td>\n                                                    <td class="icon-padding" align="center">\n                                                        <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABBAY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=bls60ivcdfptqnaijarrtqtoueaa90rih9jcafn8f2gphj7n7qig" target="_blank" style="display:block; margin:0; padding:0; line-height:0;">\n                                                            <img src="https://i1.adis.ws/i/boohooamplience/ng_em_icon_snapchat.png" alt="Snapchat" width="42" style="border-width:0; width:100%; max-width:42px; height:auto;">\n                                                        </a>\n                                                    </td>\n                                                </tr>\n                                            </table>\n\n                                            <!--[if (gte mso 9)|(IE)]>\n                                                    </td>\n                                                </tr>\n                                            </table>\n                                            <![endif]-->\n                                        </td>\n                                    </tr>\n\n                                    <tr>\n                                        <td align="center" valign="center" style="padding: 10px 4% 0 4%; font-size:0;">\n                                            <!--[if (gte mso 9)|(IE)]>\n                                            <table align="center" border="0" cellspacing="0" cellpadding="0" width="280">\n                                                <tr>\n                                                    <td align="center" valign="center" width="280">\n                                            <![endif]-->\n\n                                            <table align="center" border="0" cellpadding="0" cellspacing="0" width="100%" style="width: 100%; max-width:280px;">\n                                                <tr>\n                                                    <td align="center" width="50%" style="padding:0;">\n                                                        <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABBCY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=5on5d1nrsminfnbcqejmcvjbbp4bdav6bpghmv3h3guc2n4vq8fg" target="_blank" border="0" style="display:block; outline:none; margin:0; padding:0 5%; line-height:0;">\n                                                            <img src="https://i1.adis.ws/i/boohooamplience/ng_em_icon_app_store.png" alt="App Store" width="100" style="display:block; border-width:0; width:100%; max-width:120px; height:auto;">\n                                                        </a>\n                                                    </td>\n                                                    <td align="center" width="50%" style="padding:0;">\n                                                        <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABCRY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=ir3j6edi996e8gar8faaq4gbg67d50p670n4jki495l1a2ii4m5g" target="_blank" border="0" style="display:block; outline:none; margin:0; padding:0 5%; line-height:0;">\n                                                            <img src="https://i1.adis.ws/i/boohooamplience/ng_em_icon_google_play.png" alt="Google Play" width="100" style="display:block; border-width:0; width:100%; max-width:120px; height:auto;">\n                                                        </a>\n                                                    </td>\n                                                </tr>\n                                            </table>\n                                            \n                                            <!--[if (gte mso 9)|(IE)]>\n                                                    </td>\n                                                </tr>\n                                            </table>\n                                            <![endif]-->\n                                        </td>\n                                    </tr>\n                                </table>\n                            </div>\n                            <!-- ? END: APP ICONS -->\n\n                            <!-- # UNSUBSCRIBE COPY -->\n                            <div style="display:block;">\n                                <table class="footer" align="center" border="0" cellpadding="0" cellspacing="0" width="100%" style="max-width:700px;">\n                                    <tr>\n                                        <td align="center" style="padding:3% 0 0 0; text-align:center;">\n                                            <table class="footer" align="center" border="0" cellpadding="0" cellspacing="0" width="100%" style="max-width:700px;">\n                                                <tr>\n                                                    <td align="center" style="padding:15px 0 0 0; text-align:center;">\n                                                        <p class="copy_legal" style="font-family: \'NG-Grotesque\', Helvetica, Arial, sans-serif; color: #999999;">Score up to 80% off everything + an extra 5% off. Use code TAKE5. Valid 13.12.20 for UK only.\n                                                        <br>\n                                                        <br>\n                                                            *To qualify for a free gift you must spend £45, choose 1 product from the free gift category and enter code XMASTREAT at checkout. Valid 13.12.20 for UK only.\n                                                        </p>\n                                                    </td>\n                                                </tr>\n                                                <tr>\n                                                    <td align="center" style="padding:15px 0 0 0; text-align:center;">\n                                                        <p class="copy_legal" style="font-family:\'NG-Grotesque\', Helvetica, Arial, sans-serif; font-weight:300; color:#999999;">\n                                                            <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABCTY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=mojde2d19rvv4ii0v5rlf978vu2h9lo0mlch4ee15hd9h0a4vek0" style="color:#999999; text-decoration:none;">Contact Us</a>\n                                                            <span style="color: #999999;">&nbsp;&bull;&nbsp;</span>\n                                                            <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABCWY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=p8g0op2prl6o3tt4s0dkjh052kit0c1h5lq347rik56ebn7eqrj0" style="color:#999999; text-decoration:none;">Privacy Notice</a>\n                                                            <span style="color: #999999;">&nbsp;&bull;&nbsp;</span>\n                                                            <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABCCY&amp;_ei_=EiwPQ42l-mBFJGD0ZPxQdvmMwp2_P3883g3SgIn45kl11N0wWt6_E46BLadSi_dmqTgkgCDvFrP6oFwUTMtZD-IHKqr1E8-mdpnOblAZKvJVGkWMs1yh6iemhPRqbzzMTgaV-o8wDBF7ZoMXwgi79Iukt4ppnBRaUNE.&amp;_di_=a4mis16va320du0667rrkl5rc8456kop00053ja2lurr7ggecjkg" style="color:#999999; text-decoration:none;">Unsubscribe</a>\n                                                            <span style="color: #999999;">&nbsp;&bull;&nbsp;</span>\n                                                            <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABCAY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=eg1opg7loncukf8494r3vgeimqer58t35mc5n26dkptf10nhrlt0" style="color:#999999; text-decoration:none;">View in browser</a>\n                                                        </p>\n                                                    </td>\n                                                </tr>\n                                                <tr>\n                                                    <td align="center" style="padding:15px 0 0 0; text-align:center;">\n                                                        <p class="copy_legal" style="font-family:\'NG-Grotesque\', Helvetica, Arial, sans-serif; color:#999999;">This email was sent by: Nasty Gal A Company registered in California<br/> {NO. 10487954 } Registered office: 2135 Bay St, Los Angeles, CA 90021</p>\n                                                    </td>\n                                                </tr>\n                                                <tr>\n                                                    <td align="center" style="padding:20px 0 20px 0; text-align:center;">\n                                                        <a href="https://emailnastygal.com/pub/cc?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXtpKX%3DTSWWABWRY&amp;_ei_=Eq2tf9zs59idfPO1Sc_9BbktrKmOc1A80BaW6MY63Ka206J-yGF-U7usJq_prM.&amp;_di_=m0oj3n5jg9cvb3rkosk0qekegvb7ej69ur7c2tualdrnareu4kng" target="_blank">\n                                                            <img src="https://i1.adis.ws/i/boohooamplience/ng_em_icon.png" alt="Nasty Gal" width="30" border="0" style="border:0;" />\n                                                        </a>\n                                                    </td>\n                                                </tr>\n                                            </table>\n                                        </td>\n                                    </tr>\n                                </table>\n                            </div>\n                            <!-- ? END: UNSUBSCRIBE COPY -->\n\n                        </td>\n                    </tr>\n                </table>\n                <!--[if (gte mso 9)|(IE)]>\n                        </td>\n                    </tr>\n                </table>\n                <![endif]-->\n            </td>\n        </tr>\n        <!-- * END: FOOTER -->\n\n    </table><table cellpadding="0" cellspacing="0" style="border: 0px; padding: 0px; margin: 0px; position: absolute; display: none; float: left">\n<tr>\n<td height="1" style="font-size: 1px; line-height: 1px; padding: 0px;">\n<br><img src="https://emailnastygal.com/pub/as?_ri_=X0Gzc2X%3DAQpglLjHJlYQGtzdlFzf4PAvMbODslSzb8zcm2gpIb8zbsCutTePzcUjJD2zg1zc4m28s2ynHzaGVXHkMX%3Dw&_ei_=EolaGGF4SNMvxFF7KucKuWNjwOLyncDiCbA_Akyj-vMWiVKaEzA."></img>\n</td>\n</tr>\n</table>\n\n</body>\n\n</html>`;
