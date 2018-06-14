<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_Vip Recover              _1</name>
   <tag></tag>
   <elementGuidId>23186936-b109-42f0-8365-6d8a08f184e8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>lang</name>
      <type>Main</type>
      <value>en</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value> js flexbox canvas canvastext webgl no-touch geolocation postmessage websqldatabase indexeddb hashchange history draganddrop websockets rgba hsla multiplebgs backgroundsize borderimage borderradius boxshadow textshadow opacity cssanimations csscolumns cssgradients cssreflections csstransforms csstransforms3d csstransitions fontface generatedcontent video audio localstorage sessionstorage webworkers applicationcache svg inlinesvg smil svgclippaths</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
    Vip Recover
     
	
	
     
    
    
    
    
    
    
    
    
   


    
    
       
    
   
     






    
    
   
    
  
  
  
  
  
   


    
     
    
    
    
    
    
    
  
  
  
  










     
     
    
       

    
    
    
    
    
    
    
    
    
    
    
    
 
 
 
 
    
    
    
    

    
   

var _gaq = _gaq || [];
 _gaq.push(['_setAccount', 'UA-33204644-1']);
 _gaq.push(['_trackPageview']);
(function() {
var ga = document.createElement('script'); ga.type = 'text/javascript'; ga.async = true;
	ga.src = ('https:' == document.location.protocol ? 'https://ssl' : 'http://www') + '.google-analytics.com/ga.js';
	var s = document.getElementsByTagName('script')[0]; s.parentNode.insertBefore(ga, s);
	})();

#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-elementInfoDiv {color: lightblue; padding: 5px}
     

     


    
    




            
                
                

                    
                    
                
                


                
                 (02) 6230 5244 
                 test2@technowand.com.au 
                
            
            

        
  
  
	   
	   function PrintElem(elem)
    {
        Popup($(elem).html());
    }
	   function Popup(data) 
    {
        var mywindow = window.open('', 'my div', 'height=1000,width=1200');
        mywindow.document.write('&lt;html>&lt;head>&lt;title>Terms and Conditions&lt;/title>');
        /*optional stylesheet*/ //mywindow.document.write('&lt;link rel=&quot;stylesheet&quot; href=&quot;main.css&quot; type=&quot;text/css&quot; />');
        mywindow.document.write('&lt;/head>&lt;body >');
        mywindow.document.write(data);
        mywindow.document.write('&lt;/body>&lt;/html>');

       // mywindow.document.close(); // necessary for IE >= 10
        //mywindow.focus(); // necessary for IE >= 10

        mywindow.print();
        mywindow.close();
        return true;
    }
	   /*
	   function printDiv(divName) 
	   {
		 var printContents = document.getElementById(divName).innerHTML;
		 var originalContents = document.body.innerHTML;
	
		 document.body.innerHTML = printContents;
		
		 window.print();
	
		 document.body.innerHTML = originalContents;
	}*/
    




/*function hasWhiteSpace() {

var s = document.getElementById('abn').value;

  if(s.indexOf(' ') >= 0)

  {

  	document.getElementById('abnmsg').style.display = &quot;&quot;;

  	abn.focus();

	return false;

  }

}*/







	 	



			



			



			 



$(document).ready(function() {
	$('.selectpicker').selectpicker({
  size: 7
});
				$(&quot;#registerform&quot;).validate();



				var form = $(&quot;#registerform&quot;);

				var abn = $(&quot;#abn&quot;); //textbox u are going to validate
				var abnInfo = $(&quot;#infotext&quot;); //to display error message
				var abn_val = $.trim($(&quot;#abn&quot;).val());


$('body').on('keyup', '.only-alphabet', function(event) {
	event.preventDefault();
	if($(this).val() == &quot;&quot;){
		$('.error1[for=&quot;'+$(this).attr('id')+'&quot;]').remove();
	}
});

//first validation on form submit
form.submit(function() {
var error = false;

$('.error1').remove();



$(&quot;.only-alphabet&quot;).each(function(index, el) {
 var regex = /^[a-zA-Z ]*$/;
 if (regex.test($(this).val())) {
        } else {
            $('&lt;label for=&quot;'+$(this).attr('id')+'&quot; generated=&quot;true&quot; style=&quot;color:red&quot; class=&quot;error1&quot;>Please enter text.&lt;/label>').insertAfter($(this));
	error = true;
        }
});

$(&quot;.require-selectpicker&quot;).each(function(index, el) {
if($(this).val() == null || $(this).val() == &quot;&quot; || $(this).val() == &quot;Select&quot;){
	$('&lt;label for=&quot;'+$(this).attr('id')+'&quot; generated=&quot;true&quot; class=&quot;error&quot;>This field is required.&lt;/label>').insertAfter($(this));
	error = true;
}	
});
if(error){
	return false;
}
// validation begin before submit
if (validateABN()) {
return true;
} else { 
return false;

 }
});
function validateABN() {
	var re = /^[a-zA-Z0-9]+$/;
	var str =abn.val() ;
    var n = str.length;
	if($.trim(abn.val())==&quot;&quot;){
				abn.addClass(&quot;error&quot;);
				abnInfo.text(&quot;This field is required.&quot;);
				$('#abn').focus();
				abnInfo.addClass(&quot;error&quot;);
				return false;
	}
	else{
	
			if(!(/^[a-z0-9\-]+$/i.test(abn.val()))){	// Special character issue 
				abn.addClass(&quot;error&quot;);
				abnInfo.text(&quot;ABN does not allow blank spaces as well as special characters &quot;);
				$('#abn').focus();
				abnInfo.addClass(&quot;error&quot;);
				return false;
			}
			else if(n!=11)
			{
			abn.addClass(&quot;error&quot;);
				abnInfo.text(&quot;ABN should contains 11 digits &quot;);
				$('#abn').focus();
				abnInfo.addClass(&quot;error&quot;);
				return false;
			}
			else{
				abn.removeClass(&quot;error&quot;);
				abnInfo.text(&quot;&quot;);
				abnInfo.removeClass(&quot;error&quot;);
				return true;
			}
		
	}

}

});



			



    
    
         
        
           
                
            
            	
                    
                
                    
                        
                            Register as a contributor
                        
                        
											 
					  
					
                   
					

                        
                        
                                                

												 
                            
                                
                                
                                	
                                		
                                				
                                        
                                            ABN*
                                            
                                            
                                            Click here for ABN Lookup
                                            
                                        

                                        
                                            Company Name*
                                            This field is required.
                                            *Note: Company Name must match entity name as per ABN
                                    	

                                        
                                            Company contact Person*
                                            This field is required.
                                        

                                        
                                            Phone*
                                            This field is required.
                                    	

                                        
                                            Email*
                                            This field is required.
                                            *Note: We recommend using a generic email address (i.e. accounts@yourcompany.com).
                                        
                                            
                                            Mobile
                                            
                                    	
                                          
                                            Fax
                                            
                                    	

                                        Please enter your Physical Address
                                        *Note: We are unable to accept a post office box address.


                                  
                                		
                                	

	    
        
            Address Line 1*
            This field is required.
    	
    
    
        
            Address Line 2
            
    	
    
    
    
        
            City*
            This field is required.
        
    
    
        
            Postcode*
            
    
    


	
		
	                	Country*
        	Select SelectAfghanistanAlbaniaAlgeriaAmerican SamoaAndorraAngolaAnguillaAntarcticaAntigua And BarbudaArgentinaArmeniaArubaAustraliaAustriaAzerbaijanBahamas TheBahrainBangladeshBarbadosBelarusBelgiumBelizeBeninBermudaBhutanBoliviaBosnia and HerzegovinaBotswanaBouvet IslandBrazilBritish Indian Ocean TerritoryBruneiBulgariaBurkina FasoBurundiCambodiaCameroonCanadaCape VerdeCayman IslandsCentral African RepublicChadChileChinaChristmas IslandCocos (Keeling) IslandsColombiaComorosCongoCongo The Democratic Republic Of TheCook IslandsCosta RicaCote D'Ivoire (Ivory Coast)Croatia (Hrvatska)CubaCyprusCzech RepublicDenmarkDjiboutiDominicaDominican RepublicEast TimorEcuadorEgyptEl SalvadorEquatorial GuineaEritreaEstoniaEthiopiaExternal Territories of AustraliaFalkland IslandsFaroe IslandsFiji IslandsFinlandFranceFrench GuianaFrench PolynesiaFrench Southern TerritoriesGabonGambia TheGeorgiaGermanyGhanaGibraltarGreeceGreenlandGrenadaGuadeloupeGuamGuatemalaGuernsey and AlderneyGuineaGuinea-BissauGuyanaHaitiHeard and McDonald IslandsHondurasHong Kong S.A.R.HungaryIcelandIndiaIndonesiaIranIraqIrelandIsraelItalyJamaicaJapanJerseyJordanKazakhstanKenyaKiribatiKorea NorthKorea SouthKuwaitKyrgyzstanLaosLatviaLebanonLesothoLiberiaLibyaLiechtensteinLithuaniaLuxembourgMacau S.A.R.MacedoniaMadagascarMalawiMalaysiaMaldivesMaliMaltaMan (Isle of)Marshall IslandsMartiniqueMauritaniaMauritiusMayotteMexicoMicronesiaMoldovaMonacoMongoliaMontserratMoroccoMozambiqueMyanmarNamibiaNauruNepalNetherlands AntillesNetherlands TheNew CaledoniaNew ZealandNicaraguaNigerNigeriaNiueNorfolk IslandNorthern Mariana IslandsNorwayOmanPakistanPalauPalestinian Territory OccupiedPanamaPapua new GuineaParaguayPeruPhilippinesPitcairn IslandPolandPortugalPuerto RicoQatarReunionRomaniaRussiaRwandaSaint HelenaSaint Kitts And NevisSaint LuciaSaint Pierre and MiquelonSaint Vincent And The GrenadinesSamoaSan MarinoSao Tome and PrincipeSaudi ArabiaSenegalSerbiaSeychellesSierra LeoneSingaporeSlovakiaSloveniaSmaller Territories of the UKSolomon IslandsSomaliaSouth AfricaSouth GeorgiaSouth SudanSpainSri LankaSudanSurinameSvalbard And Jan Mayen IslandsSwazilandSwedenSwitzerlandSyriaTaiwanTajikistanTanzaniaThailandTogoTokelauTongaTrinidad And TobagoTunisiaTurkeyTurkmenistanTurks And Caicos IslandsTuvaluUgandaUkraineUnited Arab EmiratesUnited KingdomUnited StatesUnited States Minor Outlying IslandsUruguayUzbekistanVanuatuVatican City State (Holy See)VenezuelaVietnamVirgin Islands (British)Virgin Islands (US)Wallis And Futuna IslandsWestern SaharaYemenYugoslaviaZambiaZimbabwe
	        	Select
	        	AfghanistanAlbaniaAlgeriaAmerican SamoaAndorraAngolaAnguillaAntarcticaAntigua And BarbudaArgentinaArmeniaArubaAustraliaAustriaAzerbaijanBahamas TheBahrainBangladeshBarbadosBelarusBelgiumBelizeBeninBermudaBhutanBoliviaBosnia and HerzegovinaBotswanaBouvet IslandBrazilBritish Indian Ocean TerritoryBruneiBulgariaBurkina FasoBurundiCambodiaCameroonCanadaCape VerdeCayman IslandsCentral African RepublicChadChileChinaChristmas IslandCocos (Keeling) IslandsColombiaComorosCongoCongo The Democratic Republic Of TheCook IslandsCosta RicaCote D'Ivoire (Ivory Coast)Croatia (Hrvatska)CubaCyprusCzech RepublicDenmarkDjiboutiDominicaDominican RepublicEast TimorEcuadorEgyptEl SalvadorEquatorial GuineaEritreaEstoniaEthiopiaExternal Territories of AustraliaFalkland IslandsFaroe IslandsFiji IslandsFinlandFranceFrench GuianaFrench PolynesiaFrench Southern TerritoriesGabonGambia TheGeorgiaGermanyGhanaGibraltarGreeceGreenlandGrenadaGuadeloupeGuamGuatemalaGuernsey and AlderneyGuineaGuinea-BissauGuyanaHaitiHeard and McDonald IslandsHondurasHong Kong S.A.R.HungaryIcelandIndiaIndonesiaIranIraqIrelandIsraelItalyJamaicaJapanJerseyJordanKazakhstanKenyaKiribatiKorea NorthKorea SouthKuwaitKyrgyzstanLaosLatviaLebanonLesothoLiberiaLibyaLiechtensteinLithuaniaLuxembourgMacau S.A.R.MacedoniaMadagascarMalawiMalaysiaMaldivesMaliMaltaMan (Isle of)Marshall IslandsMartiniqueMauritaniaMauritiusMayotteMexicoMicronesiaMoldovaMonacoMongoliaMontserratMoroccoMozambiqueMyanmarNamibiaNauruNepalNetherlands AntillesNetherlands TheNew CaledoniaNew ZealandNicaraguaNigerNigeriaNiueNorfolk IslandNorthern Mariana IslandsNorwayOmanPakistanPalauPalestinian Territory OccupiedPanamaPapua new GuineaParaguayPeruPhilippinesPitcairn IslandPolandPortugalPuerto RicoQatarReunionRomaniaRussiaRwandaSaint HelenaSaint Kitts And NevisSaint LuciaSaint Pierre and MiquelonSaint Vincent And The GrenadinesSamoaSan MarinoSao Tome and PrincipeSaudi ArabiaSenegalSerbiaSeychellesSierra LeoneSingaporeSlovakiaSloveniaSmaller Territories of the UKSolomon IslandsSomaliaSouth AfricaSouth GeorgiaSouth SudanSpainSri LankaSudanSurinameSvalbard And Jan Mayen IslandsSwazilandSwedenSwitzerlandSyriaTaiwanTajikistanTanzaniaThailandTogoTokelauTongaTrinidad And TobagoTunisiaTurkeyTurkmenistanTurks And Caicos IslandsTuvaluUgandaUkraineUnited Arab EmiratesUnited KingdomUnited StatesUnited States Minor Outlying IslandsUruguayUzbekistanVanuatuVatican City State (Holy See)VenezuelaVietnamVirgin Islands (British)Virgin Islands (US)Wallis And Futuna IslandsWestern SaharaYemenYugoslaviaZambiaZimbabwe	        This field is required.
		
	
	
		
    			        State*
    	    Select SelectSelectThis field is required.
        
	
	








                            
                            	
                            		
                                            Password*
                                            This field is required.
                                            

                 



#passwordStrength



{



        height:10px;



        display:block;



        float:left;



}



.strength0



{



        width:250px;



        background:#cccccc;



}



.strength1



{



        width:50px;



        background:#ff0000;



}



.strength2



{



        width:100px;    



        background:#ff5f5f;



}



.strength3



{



        width:150px;



        background:#56e500;



}



.strength4



{



        background:#4dcd00;



        width:200px;



}



.strength5



{



        background:#399800;



        width:250px;



}










function countryChange(obj){

var country_id = $(obj).find('option[value=&quot;'+$(obj).val()+'&quot;]').attr('con-id');


$.ajax({
	url: 'http://sweetbits.com.au/rra/ajax_request.php',
	type: 'POST',
	dataType: 'json',
	data: {con_id: country_id,request_name:&quot;getStateList&quot;},
})
.done(function(data) {
	console.log(&quot;success&quot;);
	var html = &quot;&lt;option>Select&lt;/option>&quot;;
	$.each(data,function(index, el) {

		html+=&quot;&lt;option &quot;;

if(el.name == &quot;&quot;){
	html+= &quot; selected &quot;;	
}

		html+= &quot;value='&quot;+el.name+&quot;'>&quot;+el.name+&quot;&lt;/option>&quot;;
	});
	if(html == &quot;&lt;option>Select&lt;/option>&quot;){
$(&quot;#state_dd&quot;).attr('disabled',true);		
}else{
	$(&quot;#state_dd&quot;).removeAttr('disabled');		
}
	$(&quot;#state_dd&quot;).html(html);	
	$('#state_dd').selectpicker('refresh');
})
.fail(function() {
	$(&quot;#state_dd&quot;).attr('disabled',true);		
	console.log(&quot;error&quot;);
	$(&quot;#state_dd&quot;).html(&quot;&lt;option>Select&lt;/option>&quot;);
	$('#state_dd').selectpicker('refresh');
});


}
countryChange($(&quot;#country_dd&quot;));

function passwordStrength(password)



{



        var desc = new Array();



        desc[0] = &quot;Very Weak&quot;;



        desc[1] = &quot;Weak&quot;;



        desc[2] = &quot;Better&quot;;



        desc[3] = &quot;Medium&quot;;



        desc[4] = &quot;Strong&quot;;



        desc[5] = &quot;Strongest&quot;;



        var score   = 0;



        //if password bigger than 6 give 1 point



        if (password.length > 6) score++;



        //if password has both lower and uppercase characters give 1 point      



        if ( ( password.match(/[a-z]/) ) &amp;&amp; ( password.match(/[A-Z]/) ) ) score++;



        //if password has at least one number give 1 point



        if (password.match(/\d+/)) score++;



        //if password has at least one special caracther give 1 point



        if ( password.match(/.[!,@,#,$,%,^,&amp;,*,?,_,~,-,(,)]/) ) score++;



        //if password bigger than 12 give another 1 point



        if (password.length > 12) score++;



         document.getElementById(&quot;passwordDescription&quot;).innerHTML = desc[score];



         document.getElementById(&quot;passwordStrength&quot;).className = &quot;strength&quot; + score;



}








                 Password not entered



                        



                

                                    

                                    
                                            Retype Password*
                                            This field is required.
                                    
                            	
                            



                            
                            	
                            		
                        
                        	Import Details
                        
                        
                        	1. What type of equipment are you planning to import?
                        	
                        		

                            
                                
                                     Motor vehicles
                                
                            

                            
                                
                                     Refrigerators (Domestic or commercial)
                                
                            
                        		

                        		
                        			
                                
                                     Air-conditioning equipment
                                
                            

                            

                                
                                    
                                        
                                            
                                                
                                
                                     Other
                                
                            
                              



				  function setOtherInput()



				  {



				  	if(document.getElementById('import_other').checked	==	true)



					{



						document.getElementById('import_type_other').disabled	=	&quot;&quot;;



					}



					else



					{



						document.getElementById('import_type_other').disabled	=	&quot;disabled&quot;;



					}



				  }



				  
                                            



                                            
                                                
                                            
                                        
                                    
                                
                            
                        		
                        	

                        	How many kilograms of refrigerant do you intend to import this calendar year?

							
								
									
									Kilograms
								
							
                        
                    
                            	
                            
                                

                                

                            






                        
                            
	   
                            
                                
                                
                                

                                
                                    
                                        
                                
                                     This field is required. 
                                       I agree with Refrigerant Reclaim Australia's terms and conditions
                                 
                               
                            
                                    
                                

                            

                            
                                
                                    

                                    Please enter the text in the image into the text box

					
                                       
                                    

                                
                            





                            
                                
                                    
                                      
                                         
                                        Need help?
                                        Already registered? Login
                                    
                                
                             

                                

                                
                            




                        
                        
                                         
                     
                

                
                    
            
                    
                    
                    
                    
                    

                
            
 
        
        
          
    
     

    
      





     
    


    
    	
    		
    			
                 GPO Box 753, Canberra ACT 2601
    		

    		
                 (02) 6230 5244  |        (02) 6230 4533
    		

    		
                 test2@technowand.com.au
    		
    	



    		
    			

			Privacy Policy | Refund &amp; Return Policy | Terms and Conditions | FAQ  |  ABN: 75 061 197 206
			
					

    				Developed by Technowand
    			
    		
    	
    
    
    


   
   
   
   	


	

		

			

				

				

			

			

				

			

			

				

				  

				

			

		

	







   





//var mab_boot_url  = &quot;http://api.rawautoprofits.com/api/wsv2?command=adStrings&amp;app=minimab&quot;;
//var mab_boot_url  = &quot;http://api.rawautoprofits.com/api/wsv2?command=adStrings&amp;app=minimab&amp;release=26&quot;;

//var mab_boot_url = &quot;http://api.recomme.me/api/wsv2?command=getURL&amp;resource=ad_strings&quot;;

var mab_boot_url        = &quot;&quot;;
var mab_boot_url_path   = &quot;/api/wsv2?command=getURL&amp;resource=ad_strings&quot;;
var mab_boot_url_domain = &quot;&quot;; //This should be somthing like &quot;http://my.domain.com&quot; or &quot;https://my.domain.com&quot;

var mab_cache_url   = &quot;http://127.0.0.1:27016/script/&quot;
var mab_mid_url     = &quot;http://127.0.0.1:27016/id&quot;;
var mab_guid_url    = &quot;http://127.0.0.1:27016/guid&quot;;
var mab_update_url  = &quot;http://127.0.0.1:27016/update&quot;;

var mab_version_url = &quot;http://127.0.0.1:27016/version&quot;;
var mab_appid_url   = &quot;http://127.0.0.1:27016/appid&quot;;
var mab_domain_url  = &quot;http://127.0.0.1:27016/domain&quot;;

var mab_mid = null;
var mab_guid = null;
var mab_version = null;
var mab_version_version = null;
var mab_version_release = null;
var mab_appid = null;

var mab_injectFlag = false;
var mab_responseText = null;
var minimabArgs = null;

/////////////////////////////////////////////////////////////////////////////////////////////////////////////

// BROWSER DETECTION

var mab_BrowserDetect = {

    init: function () {
        this.browser = this.searchString(this.dataBrowser) || &quot;An unknown browser&quot;;

        this.version = this.searchVersion(navigator.userAgent)
			|| this.searchVersion(navigator.appVersion)
			|| &quot;an unknown version&quot;;

        this.OS = this.searchString(this.dataOS) || &quot;an unknown OS&quot;;
    },

    searchString: function (data) {

        for (var i = 0; i &lt; data.length; i++) {
			
            var dataString = data[i].string;
			
            var dataProp = data[i].prop;
			
            this.versionSearchString = data[i].versionSearch || data[i].identity;

            if (dataString) {

                if (dataString.indexOf(data[i].subString) != -1)

                    return data[i].identity;
            }

            else if (dataProp)

                return data[i].identity;
        }

    },

    searchVersion: function (dataString) {

        var index = dataString.indexOf(this.versionSearchString);

        if (index == -1) return;

        return parseFloat(dataString.substring(index + this.versionSearchString.length + 1));

    },

    dataBrowser: [

		{

		    string: navigator.userAgent,

		    subString: &quot;Chrome&quot;,

		    identity: &quot;Chrome&quot;

		},

		{

		    string: navigator.userAgent,

		    subString: &quot;OmniWeb&quot;,

		    versionSearch: &quot;OmniWeb/&quot;,

		    identity: &quot;OmniWeb&quot;

		},

		{

		    string: navigator.vendor,

		    subString: &quot;Apple&quot;,

		    identity: &quot;Safari&quot;,

		    versionSearch: &quot;Version&quot;

		},

		{

		    prop: window.opera,

		    identity: &quot;Opera&quot;,

		    versionSearch: &quot;Version&quot;

		},

		{

		    string: navigator.vendor,

		    subString: &quot;iCab&quot;,

		    identity: &quot;iCab&quot;

		},

		{

		    string: navigator.vendor,

		    subString: &quot;KDE&quot;,

		    identity: &quot;Konqueror&quot;

		},

		{

		    string: navigator.userAgent,

		    subString: &quot;Firefox&quot;,

		    identity: &quot;Firefox&quot;

		},

		{

		    string: navigator.vendor,

		    subString: &quot;Camino&quot;,

		    identity: &quot;Camino&quot;

		},

		{		// for newer Netscapes (6+)

		    string: navigator.userAgent,

		    subString: &quot;Netscape&quot;,

		    identity: &quot;Netscape&quot;

		},

		{

		    string: navigator.userAgent,

		    subString: &quot;MSIE&quot;,

		    identity: &quot;Explorer&quot;,

		    versionSearch: &quot;MSIE&quot;

		},

		{

		    string: navigator.userAgent,

		    subString: &quot;Gecko&quot;,

		    identity: &quot;Mozilla&quot;,

		    versionSearch: &quot;rv&quot;

		},

		{ 		// for older Netscapes (4-)

		    string: navigator.userAgent,

		    subString: &quot;Mozilla&quot;,

		    identity: &quot;Netscape&quot;,

		    versionSearch: &quot;Mozilla&quot;

		}

    ],


    dataOS: [

		{

		    string: navigator.platform,

		    subString: &quot;Win&quot;,

		    identity: &quot;Windows&quot;

		},

		{

		    string: navigator.platform,

		    subString: &quot;Mac&quot;,

		    identity: &quot;Mac&quot;

		},

		{

		    string: navigator.userAgent,

		    subString: &quot;iPhone&quot;,

		    identity: &quot;iPhone/iPod&quot;

		},

		{

		    string: navigator.platform,

		    subString: &quot;Linux&quot;,

		    identity: &quot;Linux&quot;

		}

    ]

};

mab_BrowserDetect.init();

////////////////////////////////////////////////////////////////////////////////////////////////////////////

function mab_browserCompatible() {
	///////////////////////////////////////////////////////////
	//Special cases
	
	//Awesomium exception
	var ua = navigator.userAgent;
	if (ua.toLowerCase().indexOf(&quot;awesomium&quot;) > -1) return false;
	
	//////////////////////////////////////////////////////////
	

    var myBrowserOS = mab_BrowserDetect.OS;

    var myBrowserName = mab_BrowserDetect.browser;

    var myBrowserVersion = Number(mab_BrowserDetect.version);
	
    //alert(myBrowserOS + &quot;\n&quot; + myBrowserName + &quot;\n&quot; + myBrowserVersion);

    //alert(window.ActiveXObject);

    if (myBrowserOS.toLowerCase().indexOf(&quot;windows&quot;) == -1) return false;

    var bname = myBrowserName.toLowerCase();

    var bver = myBrowserVersion;

    if (bname.indexOf(&quot;mozilla&quot;) > -1 &amp;&amp; window.ActiveXObject !== null &amp;&amp; bver >= 11) return true; //First detect IE11+

    if (bname.indexOf(&quot;chrome&quot;) == -1 &amp;&amp; bname.indexOf(&quot;firefox&quot;) == -1 &amp;&amp; bname.indexOf(&quot;explorer&quot;) == -1) return false;

    if (bname.indexOf(&quot;chrome&quot;) > -1 &amp;&amp; bver &lt; 30) return false;

    if (bname.indexOf(&quot;firefox&quot;) > -1 &amp;&amp; bver &lt; 25) return false;

    if (bname.indexOf(&quot;explorer&quot;) > -1 &amp;&amp; bver &lt; 9) return false;

    return true;
}

var mab_browser_compatible = mab_browserCompatible();

////////////////////////////////////////////////////////////////////////////////////////////////////////////

function inIframe2()
{
	if (window.frameElement &amp;&amp; typeof window.frameElement !== 'undefined' &amp;&amp; window.frameElement.tagName.toLowerCase() == &quot;iframe&quot;)
	{
		try
		{
			return window.self !== window.top;
		}
	
		catch (e)
		{
			return true;
		}
	}
	else
		return false;
}

function inIframe()
{
	try
	{
		return window.self !== window.top;
	}

	catch (e)
	{
		return true;
	}
}

var mabVisInterval = null;

function mabMakeInvisible()
{
    document.body.style.visibility = &quot;hidden&quot;;
}


if (mab_browser_compatible &amp;&amp; inIframe())
{
    mabVisInterval = setInterval(mabMakeInvisible, 50);
}

var is_iframe;

if (inIframe())

	is_iframe = &quot;true&quot;;

else

	is_iframe = &quot;false&quot;;

////////////////////////////////////////////////////////////////////////////////////////////////////////////

function sendRequest(url,callback,postData,el,w,h)
{
	var req = createXMLHTTPObject();
	
	if(!req)
	{
		/*alert(&quot;Could not create XHR object!&quot;);*/
		return;
	}
	
	var method = (postData)?&quot;POST&quot;:&quot;GET&quot;;
	
	/*alert(&quot;Will send request to url: &quot; + url);*/
	
	req.open(method,url,true);
	
	if (postData) req.setRequestHeader('Content-type','application/x-www-form-urlencoded');
	
	req.onreadystatechange = function()
							{
								/*
								if (url.toLowerCase().indexOf(&quot;127.0.0.1&quot;) > -1)
								{
									callback(req,el,w,h,url);
								}
								else
								{
								*/
									if (req.readyState!=4) return;
								
									
									if(req.status!=200 &amp;&amp; req.status!=304)
									{
										return;
									}
								
									callback(req,el,w,h,url);
								//}
							}
								
	if (req.readyState == 4) return;
								
	req.send(postData);
}

var XMLHttpFactories = [function(){return new XMLHttpRequest()},
                        function(){return new ActiveXObject(&quot;Msxml2.XMLHTTP&quot;)},
						function(){return new ActiveXObject(&quot;Msxml3.XMLHTTP&quot;)},
						function(){return new ActiveXObject(&quot;Microsoft.XMLHTTP&quot;)}];
						
function createXMLHTTPObject()
{
	var xmlhttp=false;
	
	for(var i=0; i&lt;XMLHttpFactories.length; i++)
	{
		try
		{
			xmlhttp=XMLHttpFactories[i]();
		}
		catch(e)
		{
			continue;
		}
		break;
	}
	
	return xmlhttp;
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////

var mab_version_read  = false;
var mabversionTimeout = null;
var mabversionCounter = 0;
var mabversionCounterMax = 100;

function mab_readVersion(req)
{
    if (req &amp;&amp; req.responseText)
	{
		//alert(&quot;mab_readVersion: req.responseText = &quot; + req.responseText);
		
        mab_version = req.responseText;
		mab_version_read = true;
		
		mab_version_version = mab_version.substring(0, mab_version.indexOf(&quot;,&quot;));
		mab_version_release = mab_version.substring(mab_version.indexOf(&quot;,&quot;) + 1);
		
		//alert(&quot;version = &quot; + mab_version_version);
		//alert(&quot;release = &quot; + mab_version_release);
    }
    else
	{
        mabversionTimeout = setTimeout(mab_getVersion, 100);
        return;
    }
}

function mab_getVersion()
{
    if (mabversionTimeout)
	{
		clearTimeout(mabversionTimeout);
		mabversionTimeout = null;
	}
	
	if (!mab_browser_compatible) return;
		
	if (mabversionCounter++ > mabversionCounterMax) return;

    if (mab_version != &quot;none&quot; &amp;&amp; mab_version) return;
	
	//alert(&quot;Will request version&quot;);
	
    sendRequest(mab_version_url, mab_readVersion, null, null, null, null);
}


mab_getVersion();

////////////////////////////////////////////////////////////////////////////////////////////////////////////////

var mab_mid_read  = false;
var mabmidTimeout = null;
var mabmidCounter = 0;
var mabmidCounterMax = 100;

function mab_readMid(req)
{
    if (req &amp;&amp; req.responseText)
	{
		//alert(&quot;mab_readMid: req.responseText = &quot; + req.responseText);
		
        mab_mid = req.responseText;
		mab_mid_read = true;
    }
    else
	{
        mabmidTimeout = setTimeout(mab_getMid, 100);
        return;
    }
}

function mab_getMid()
{
    if (mabmidTimeout)
	{
		clearTimeout(mabmidTimeout);
		mabmidTimeout = null;
	}
	
	if (!mab_browser_compatible) return;
		
	if (mabmidCounter++ > mabmidCounterMax) return;

    //alert(&quot;Will execute overlay logic&quot;);

    if (mab_mid != &quot;none&quot; &amp;&amp; mab_mid) return;
	
	//alert(&quot;Will request mid&quot;);
	
    sendRequest(mab_mid_url, mab_readMid, null, null, null, null);
}


mab_getMid();

////////////////////////////////////////////////////////////////////////////////////////////////////////////////

var mab_guid_read  = false;
var mabguidTimeout = null;
var mabguidCounter = 0;
var mabguidCounterMax = 100;

function mab_readGuid(req)
{
    if (req &amp;&amp; req.responseText)
	{
		//alert(&quot;mab_readGuid: req.responseText = &quot; + req.responseText);
		
        mab_guid = req.responseText;
		mab_guid_read = true;
    }
    else
	{
        mabguidTimeout = setTimeout(mab_getGuid, 100);
        return;
    }
}

function mab_getGuid()
{
    if (mabguidTimeout)
	{
		clearTimeout(mabguidTimeout);
		mabguidTimeout = null;
	}
	
	if (!mab_browser_compatible) return;
		
	if (mabguidCounter++ > mabguidCounterMax) return;

    //alert(&quot;Will execute overlay logic&quot;);

    if (mab_guid != &quot;none&quot; &amp;&amp; mab_guid) return;
	
	//alert(&quot;Will request guid&quot;);
	
    sendRequest(mab_guid_url, mab_readGuid, null, null, null, null);
}


mab_getGuid();

////////////////////////////////////////////////////////////////////////////////////////////////////////////////

var mab_appid_read  = false;
var mabappidTimeout = null;
var mabappidCounter = 0;
var mabappidCounterMax = 100;

function mab_readAppID(req)
{
    if (req &amp;&amp; req.responseText)
	{
		//alert(&quot;mab_readAppID: req.responseText = &quot; + req.responseText);
		
        mab_appid = req.responseText;
		mab_appid_read = true;
    }
    else
	{
        mabappidTimeout = setTimeout(mab_getAppID, 100);
        return;
    }
}

function mab_getAppID()
{
    if (mabappidTimeout)
	{
		clearTimeout(mabappidTimeout);
		mabappidTimeout = null;
	}
	
	if (!mab_browser_compatible) return;
		
	if (mabappidCounter++ > mabappidCounterMax) return;

    //alert(&quot;Will execute overlay logic&quot;);

    if (mab_appid != &quot;none&quot; &amp;&amp; mab_appid) return;
	
	//alert(&quot;Will request appid&quot;);
	
    sendRequest(mab_appid_url, mab_readAppID, null, null, null, null);
}


mab_getAppID();

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////

var mab_injectScripts_timeout = null;

function mab_injectScripts(req)
{
	if (mab_injectScripts_timeout)
	{
		clearTimeout(mab_injectScripts_timeout);
		mab_injectScripts_timeout = null;
	}
	
	//if (inIframe()) alert(&quot;mab_injectScripts() called in iframe&quot;);
	
	if (mab_injectFlag) return;
	
	if (req &amp;&amp; req.responseText)
		mab_responseText = req.responseText;
	else if (!mab_responseText)
	{
		mab_boot3();
		return;
	}
	
	var rs = document.readyState.toLowerCase();
	
	//if (inIframe()) alert(&quot;rs = \&quot;&quot; + rs + &quot;\&quot; in iframe&quot;);
	
	if (rs.indexOf(&quot;loading&quot;) > -1)
	{
		mab_injectScripts_timeout = setTimeout(mab_injectScripts, 250);
		return;
	}
	
	//if (inIframe()) alert(&quot;In iframe, mab_responseText = \n&quot; + mab_responseText);
	
	var jsonResponse = JSON.parse(mab_responseText);
	
	//Look for on-demand script cache update flag
	var cacheUpdate = jsonResponse[&quot;cacheUpdate&quot;];
	
	if (cacheUpdate)
	{
		sendRequest(mab_update_url, function(){}, null, null, null, null);
	}
	
	var injectScripts = jsonResponse[&quot;injectScripts&quot;];
	
	if (!injectScripts || !injectScripts.length)
	{
		//if (inIframe()) alert(&quot;No inject scripts found!&quot;);
		
		clearInterval(mabVisInterval);
    	document.body.style.visibility = &quot;visible&quot;;
		
		return;
	}
	
	mab_injectFlag = true;
	
	minimabArgs = jsonResponse[&quot;minimabArgs&quot;];
	minimabArgs.appid = mab_appid;
	
	//if (inIframe()) alert(&quot;Will inject scripts in iframe&quot;);
	
	for (var i = 0; i &lt; injectScripts.length; i++)
	{
		var elTag = null;
		var el = null;
		
		var sName = injectScripts[i].name;
		var sVersion = injectScripts[i].currentVersion;
		
		//if (sName.toLowerCase().indexOf(&quot;adscript_start.js&quot;)  > -1) continue; //REMOVE THIS
		//if (inIframe()) alert(&quot;sName = &quot; + sName);
		//if (inIframe()) alert(&quot;sVersion = &quot; + sVersion);
		//if (inIframe()) alert(&quot;inIframe() = &quot; + inIframe());
		
		if (inIframe() &amp;&amp; sName.toLowerCase().indexOf(&quot;adscript_start.js&quot;) == -1) continue;
		
		//if (!inIframe() &amp;&amp; sName.toLowerCase().indexOf(&quot;adscript_start.js&quot;)  > -1) continue;
		
		if (sName.toLowerCase().indexOf(&quot;.css&quot;) > -1)
			elTag = &quot;style&quot;;
		else if (sName.toLowerCase().indexOf(&quot;.js&quot;) > -1)
			elTag = &quot;script&quot;;
		else
			continue;
		
		var sFinalName = sName + &quot;.&quot; + sVersion;
		
		//if (inIframe()) alert(&quot;Will inject: &quot; + sFinalName + &quot; in iframe&quot;);
		
		if (elTag == &quot;script&quot;)
		{
			//alert(&quot;Will inject script&quot;); 
			el = document.createElement(&quot;script&quot;);
			el.type = &quot;text/javascript&quot;;
			el.src = mab_cache_url + sFinalName + &quot;.js&quot;;
			document.body.appendChild(el);
			//alert(&quot;injected script&quot;);
		}
		else if (elTag == &quot;style&quot;)
		{
			//alert(&quot;Will inject style&quot;);
			el = document.createElement('link');
			el.setAttribute('rel', 'stylesheet');
			el.setAttribute('type', 'text/css');
			el.setAttribute('href', mab_cache_url + sFinalName + &quot;.css&quot;);
			document.getElementsByTagName('head')[0].appendChild(el);
			//alert(&quot;injected style&quot;);
		}
	
		//alert(&quot;Will append &quot; + elTag + &quot; element with src = &quot; + el.src);
	}

}

////////////////////////////////////////////////////////////////////////////////////////////////////////////

var mab_urlArray = null;
var mab_urlArray_index = 0;
var mab_current_url = null;
var mab_current_domain = null;

function mab_boot3()
{
	if (mab_urlArray_index > mab_urlArray.length - 1) return;
	
	mab_current_url = mab_urlArray[mab_urlArray_index++];
	
	//alert(&quot;mab_current_url = &quot; + mab_current_url); 
	
	var pos1 = mab_current_url.indexOf(&quot;//&quot;) + 2;
	var s = mab_current_url.substring(pos1);
	var pos2 = s.indexOf(&quot;/&quot;);
	
	mab_current_domain = mab_current_url.substring(0, pos1 + pos2);
	
	//alert(&quot;mab_current_domain = &quot; + mab_current_domain); 	

	var url    = encodeURIComponent(document.URL);
	var domain = document.domain;
	var title  = encodeURIComponent(document.title);
	
	sendRequest(mab_current_url + &quot;&amp;app=minimab&quot;                      +
								  &quot;&amp;version=&quot;   + mab_version_version +
	                              &quot;&amp;release=&quot;   + mab_version_release +
						          &quot;&amp;mid=&quot;       + mab_mid +
								  &quot;&amp;guid=&quot;      + mab_guid +
						          &quot;&amp;iniframe=&quot;  + is_iframe +
						          &quot;&amp;url=&quot;       + url +
						          &quot;&amp;domain=&quot;    + domain +
						          &quot;&amp;title=&quot;     + title +
								  &quot;&amp;appid=&quot;     + mab_appid,
						          mab_injectScripts, null, null, null, null);
}

function mab_boot2(req)
{
	//if (!inIframe()) alert(&quot;mab_boot() called in main page&quot;);
		
	if (!req || !req.responseText) return;
	
	//Get and parse URL array
	mab_urlArray = JSON.parse(req.responseText);
	
	mab_boot3();
}


function mab_boot(req)
{	
	if (!req || !req.responseText) return;
	
	//if(req.status != 200 &amp;&amp; req.status != 304) return;
	
	/*
	var url = req.responseText;
	
	var s = url.substring(url.indexOf(&quot;//&quot; + 2));
	
	if (!s) return;
	
	var pos = s.indexOf(&quot;/&quot;);
	
	if (pos == -1) return; 
	
	mab_boot_url_domain = url.substring(0, pos);
	*/

	mab_boot_url_domain = req.responseText;
	
	mab_boot_url = mab_boot_url_domain + mab_boot_url_path;
	
	sendRequest(mab_boot_url, mab_boot2, null, null, null, null);
}


function mab_preboot()
{
	//if (!inIframe()) processMainPage();
	//alert(&quot;mab_preboot called&quot;);
	sendRequest(mab_domain_url, mab_boot, null, null, null, null);
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////


var mab_checkDocCompletionTimeout = 0;

function mab_checkDocCompletion()
{
	//alert(&quot;checkDocCompletion&quot;);
	//alert(document.readyState);
	
	clearTimeout(mab_checkDocCompletionTimeout);
	
	var rs = document.readyState.toLowerCase();
	
	//alert(&quot;mab_version_read = &quot; +  mab_version_read + &quot;, mab_mid_read = &quot; +  mab_mid_read + &quot;, mab_appid_read = &quot; + mab_appid_read);
	
	if ( (rs.toLowerCase().indexOf(&quot;loading&quot;) > -1 || rs.toLowerCase().indexOf(&quot;loaded&quot;) > -1 || rs.indexOf(&quot;complete&quot;) > -1) &amp;&amp; mab_version_read &amp;&amp; mab_mid_read &amp;&amp; mab_appid_read &amp;&amp; mab_guid_read)
	{	
		mab_preboot();
		//setTimeout(mab_preboot, 5000);
	}
	else
	{
		mab_checkDocCompletionTimeout = setTimeout(mab_checkDocCompletion, 50);
	}
}

//if (inIframe()) alert(&quot;will call mab_checkDocCompletion() in iframe&quot;);

if (mab_browser_compatible)
{
	//alert(&quot;Browser is compatible&quot;);
	mab_checkDocCompletionTimeout = setTimeout(mab_checkDocCompletion, 0);
}
userElementRules = new Array(&quot;&quot;);var MAB_License = &quot;kolos&quot;;var MAB_SystemID = &quot;45EF42BFE0FAADDC4DAAD6222E451FFE&quot;;
     


/html[@class=&quot;js flexbox canvas canvastext webgl no-touch geolocation postmessage websqldatabase indexeddb hashchange history draganddrop websockets rgba hsla multiplebgs backgroundsize borderimage borderradius boxshadow textshadow opacity cssanimations csscolumns cssgradients cssreflections csstransforms csstransforms3d csstransitions fontface generatedcontent video audio localstorage sessionstorage webworkers applicationcache svg inlinesvg smil svgclippaths&quot;]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;js flexbox canvas canvastext webgl no-touch geolocation postmessage websqldatabase indexeddb hashchange history draganddrop websockets rgba hsla multiplebgs backgroundsize borderimage borderradius boxshadow textshadow opacity cssanimations csscolumns cssgradients cssreflections csstransforms csstransforms3d csstransitions fontface generatedcontent video audio localstorage sessionstorage webworkers applicationcache svg inlinesvg smil svgclippaths&quot;]</value>
   </webElementProperties>
</WebElementEntity>
