<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>5.- vacante3</name>
   <tag></tag>
   <elementGuidId>87b2ab05-e21b-4be9-ad1b-196e28867c8d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${GlobalVariable.TokenReclu}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;academicTitle\&quot;: \&quot;ingeniero\&quot;,\n    \&quot;area\&quot;: [\n        {\n            \&quot;exclud\&quot;: false,\n            \&quot;area\&quot;: {\n                \&quot;areaId\&quot;: \&quot;40288087797b055a01797b12ab830007\&quot;\n            },\n            \&quot;yearExperience\&quot;: 2\n        }\n    ],\n    \&quot;hardSkill\&quot;: [\n        {\n            \&quot;level\&quot;: \&quot;INTERMEDIO\&quot;,\n            \&quot;skillId\&quot;: \&quot;2c9f93647d0ba197017d14ef1b830061\&quot;,\n            \&quot;exclud\&quot;: false\n        }\n    ],\n    \&quot;idEducation\&quot;: \&quot;402881cf79c889e50179c88fc5850003\&quot;,\n    \&quot;levelEducationExclud\&quot;: false,\n    \&quot;idStatusEducation\&quot;: \&quot;402880de79730a2c0179731b7aed0002\&quot;,\n    \&quot;statusEducationExclud\&quot;: false,\n    \&quot;institution\&quot;: [],\n    \&quot;language\&quot;: [],\n    \&quot;softSkill\&quot;: [\n        {\n            \&quot;skillId\&quot;: \&quot;0000000088c520440188d5ce7c2d00fb\&quot;\n        }\n    ],\n    \&quot;speciality\&quot;: [\n        {\n            \&quot;exclud\&quot;: false,\n            \&quot;speciality\&quot;: {\n                \&quot;specialtyId\&quot;: \&quot;40288087797b055a01797b3703990021\&quot;\n            },\n            \&quot;yearExperience\&quot;: 3\n        }\n    ]\n}&quot;,
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
      <webElementGuid>e1daf8e6-3eaa-465c-afb5-f08dfbca460f</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.TokenReclu}</value>
      <webElementGuid>a5531e18-b303-4f2b-999a-3f16be1dfda5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/vacancy/management/step3/${GlobalVariable.vacanteid}</restUrl>
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
      <id>7c6fa9fa-08d5-4359-aa62-73e0f5698e03</id>
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
