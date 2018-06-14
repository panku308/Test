<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Password</name>
   <tag></tag>
   <elementGuidId>c9a79e3c-6dd3-4ca7-85d9-7345ba0ff58a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>form-group</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
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



                        



                

                                    </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;registerform&quot;)/div[@class=&quot;row&quot;]/div[@class=&quot;col-md-10&quot;]/div[@class=&quot;row&quot;]/div[@class=&quot;col-md-12&quot;]/div[@class=&quot;form-group&quot;]</value>
   </webElementProperties>
</WebElementEntity>
