<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>5.- formulario nombre</name>
   <tag></tag>
   <elementGuidId>a13a206c-56b9-4428-9143-9b075cbdc22a</elementGuidId>
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
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;[\n    {\n        \&quot;op\&quot;: \&quot;replace\&quot;,\n        \&quot;path\&quot;: \&quot;/user/name\&quot;,\n        \&quot;value\&quot;: \&quot;${nombre}\&quot;\n    },\n    {\n        \&quot;op\&quot;: \&quot;replace\&quot;,\n        \&quot;path\&quot;: \&quot;/user/lastName\&quot;,\n        \&quot;value\&quot;: \&quot;${apellidoP}\&quot;\n    },\n    {\n        \&quot;op\&quot;: \&quot;replace\&quot;,\n        \&quot;path\&quot;: \&quot;/user/secondLastName\&quot;,\n        \&quot;value\&quot;: \&quot;${apellidoM}\&quot;\n    },\n    {\n        \&quot;op\&quot;: \&quot;replace\&quot;,\n        \&quot;path\&quot;: \&quot;/steepsOnboarding\&quot;,\n        \&quot;value\&quot;: 1\n    }\n]&quot;,
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
      <webElementGuid>c4d5de7b-e214-42d6-9da5-af7d4b1b08b9</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.TokenCand}</value>
      <webElementGuid>4c1a1af8-1bcc-43f6-a178-34894840d216</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PATCH</restRequestMethod>
   <restUrl>${url}/user/candidate</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>cd23bbc1-ed7b-4804-bd96-434fcf1a612f</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Nombres</defaultValue>
      <description></description>
      <id>6ef7bb50-0ca8-4289-813d-24c7a572dcce</id>
      <masked>false</masked>
      <name>nombre</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ApellidoP</defaultValue>
      <description></description>
      <id>b91f5c3e-932d-4682-8d41-399d308ac508</id>
      <masked>false</masked>
      <name>apellidoP</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ApellidoM</defaultValue>
      <description></description>
      <id>aaecbd7d-a10e-492f-81eb-dfdb8281930b</id>
      <masked>false</masked>
      <name>apellidoM</name>
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
