<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>19.- area</name>
   <tag></tag>
   <elementGuidId>ec1bd004-1385-4c69-a309-92bef12133df</elementGuidId>
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
  &quot;text&quot;: &quot;[\n    {\n        \&quot;areaId\&quot;: \&quot;40288087797b055a01797b12ab830007\&quot;,\n        \&quot;spanishName\&quot;: \&quot;Administración / Oficina\&quot;,\n        \&quot;englishName\&quot;: \&quot;Administration\&quot;,\n        \&quot;keySystem\&quot;: \&quot;MEX\&quot;,\n        \&quot;specialties\&quot;: [\n            {\n                \&quot;specialtyId\&quot;: \&quot;40288087797b055a01797b3705d50023\&quot;,\n                \&quot;englishName\&quot;: \&quot;Office administration\&quot;,\n                \&quot;spanishName\&quot;: \&quot;Administración de oficina\&quot;,\n                \&quot;keySystem\&quot;: \&quot;MEX\&quot;,\n                \&quot;areaId\&quot;: \&quot;40288087797b055a01797b12ab830007\&quot;,\n                \&quot;isChecked\&quot;: true\n            }\n        ]\n    }\n]&quot;,
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
      <webElementGuid>0a991a52-d143-4871-aac1-5f967403ad38</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.TokenCand}</value>
      <webElementGuid>733c471d-f9f2-415b-b6de-08f0046c15d9</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://${url}.micros.involverh.com.mx/candidate/candidate/area</restUrl>
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
      <id>a485f7d6-92c8-4c64-91f3-41ca63dce326</id>
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
