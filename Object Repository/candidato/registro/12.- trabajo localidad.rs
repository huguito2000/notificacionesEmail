<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>12.- trabajo localidad</name>
   <tag></tag>
   <elementGuidId>ba16e7a4-5c15-41dd-9e52-d178fa8cbd5e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${GlobalVariable.TokenCand}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;[\n    {\n        \&quot;op\&quot;: \&quot;replace\&quot;,\n        \&quot;path\&quot;: \&quot;/steepsOnboarding\&quot;,\n        \&quot;value\&quot;: \&quot;8\&quot;\n    },\n    {\n        \&quot;op\&quot;: \&quot;replace\&quot;,\n        \&quot;path\&quot;: \&quot;/citySearch\&quot;,\n        \&quot;value\&quot;: {\n            \&quot;cityId\&quot;: \&quot;2c9f936481969f0cccc996a00e090278\&quot;,\n            \&quot;name\&quot;: \&quot;Coyoacán\&quot;,\n            \&quot;stateCode\&quot;: \&quot;MX-CMX\&quot;,\n            \&quot;countryCode\&quot;: \&quot;MX\&quot;,\n            \&quot;latitude\&quot;: 19.34678,\n            \&quot;longitude\&quot;: -99.15968,\n            \&quot;state\&quot;: {\n                \&quot;stateId\&quot;: \&quot;2c9f936481969f0bbbb996a00e090009\&quot;,\n                \&quot;name\&quot;: \&quot;Ciudad de México\&quot;,\n                \&quot;countryCode\&quot;: \&quot;MX\&quot;,\n                \&quot;fipsCode\&quot;: \&quot;9\&quot;,\n                \&quot;iso2\&quot;: \&quot;MX-CMX\&quot;,\n                \&quot;latitude\&quot;: 19.28333,\n                \&quot;longitude\&quot;: -99.13333,\n                \&quot;country\&quot;: {\n                    \&quot;countryId\&quot;: \&quot;2c9f936481969f0aaaa996a00e090001\&quot;,\n                    \&quot;capital\&quot;: \&quot;Mexico City\&quot;,\n                    \&quot;currency\&quot;: \&quot;MXN\&quot;,\n                    \&quot;currencySymbol\&quot;: \&quot;$\&quot;,\n                    \&quot;iso2\&quot;: \&quot;MX\&quot;,\n                    \&quot;iso3\&quot;: \&quot;MEX\&quot;,\n                    \&quot;latitude\&quot;: 19.4284,\n                    \&quot;longitude\&quot;: -99.1276,\n                    \&quot;name\&quot;: \&quot;Mexico\&quot;,\n                    \&quot;nameNative\&quot;: \&quot;México\&quot;,\n                    \&quot;phoneCode\&quot;: \&quot;52\&quot;,\n                    \&quot;region\&quot;: \&quot;Americas\&quot;,\n                    \&quot;subregion\&quot;: \&quot;Central America\&quot;,\n                    \&quot;timezones\&quot;: \&quot;[{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Bahia_Banderas\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-21600,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-06:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;CST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Central Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Cancun\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-18000,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-05:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;EST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Eastern Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Chihuahua\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-25200,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-07:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;MST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Mountain Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Hermosillo\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-25200,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-07:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;MST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Mountain Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Matamoros\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-21600,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-06:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;CST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Central Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Mazatlan\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-25200,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-07:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;MST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Mountain Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Merida\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-21600,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-06:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;CST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Central Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Mexico_City\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-21600,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-06:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;CST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Central Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Monterrey\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-21600,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-06:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;CST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Central Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Ojinaga\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-25200,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-07:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;MST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Mountain Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Tijuana\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-28800,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-08:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;PST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Pacific Standard Time (North America\\\&quot;}]\&quot;,\n                    \&quot;tld\&quot;: \&quot;.mx\&quot;,\n                    \&quot;translations\&quot;: \&quot;{\\\&quot;br\\\&quot;:\\\&quot;México\\\&quot;,\\\&quot;pt\\\&quot;:\\\&quot;México\\\&quot;,\\\&quot;nl\\\&quot;:\\\&quot;Mexico\\\&quot;,\\\&quot;hr\\\&quot;:\\\&quot;Meksiko\\\&quot;,\\\&quot;fa\\\&quot;:\\\&quot;مکزیک\\\&quot;,\\\&quot;de\\\&quot;:\\\&quot;Mexiko\\\&quot;,\\\&quot;es\\\&quot;:\\\&quot;México\\\&quot;,\\\&quot;fr\\\&quot;:\\\&quot;Mexique\\\&quot;,\\\&quot;ja\\\&quot;:\\\&quot;メキシコ\\\&quot;,\\\&quot;it\\\&quot;:\\\&quot;Messico\\\&quot;}\&quot;,\n                    \&quot;flagCountry\&quot;: \&quot;https://flagcdn.com/w20/mx.png\&quot;\n                }\n            }\n        }\n    }\n]&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>cd5fd115-2bc7-4e8d-9ce7-0007efb2349c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.TokenCand}</value>
      <webElementGuid>8b917d6b-d1bc-4b17-9f34-56fe2f625177</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PATCH</restRequestMethod>
   <restUrl>https://${url}.micros.involverh.com.mx/candidate/candidate</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>435d3681-d957-43a9-a29d-0959ae20ed35</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
