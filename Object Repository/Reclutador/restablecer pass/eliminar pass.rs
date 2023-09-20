<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>eliminar pass</name>
   <tag></tag>
   <elementGuidId>c24c0ce4-4637-4ce4-a96b-f00d53467008</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzUxMiJ9.eyJqdGkiOiIyYzlmOTM2NDg3NWNkNzJmMDE4NzZjMjA4NDJlMDAyZCIsInN1YiI6InJlY2x1dGFkb3IucWFAeW9wbWFpbC5jb20iLCJhdXRob3JpdGllcyI6WyJBRE1JTiJdLCJpZCI6IjJjOWY5MzY0ODc1Y2Q3MmYwMTg3NmMyMDg0MmUwMDJkIiwidXNlclJvbCI6IkFETUlOIiwiZW1haWxVc2VyIjoicmVjbHV0YWRvci5xYUB5b3BtYWlsLmNvbSIsImtleVN5c3RlbSI6Ik1FWCIsImlhdCI6MTY4NzI3Mjk5MCwiZXhwIjoxNjg3MjgzNzkwfQ._t-iGP0Ivx3L-367vLBBWFVZjlNOrzyBjTkuOFl2A2FTw5W95IlOX68Puk6o_WfhkKBcwXgawcglhXhJvNSDyA</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzUxMiJ9.eyJqdGkiOiIyYzlmOTM2NDg3NWNkNzJmMDE4NzZjMjA4NDJlMDAyZCIsInN1YiI6InJlY2x1dGFkb3IucWFAeW9wbWFpbC5jb20iLCJhdXRob3JpdGllcyI6WyJBRE1JTiJdLCJpZCI6IjJjOWY5MzY0ODc1Y2Q3MmYwMTg3NmMyMDg0MmUwMDJkIiwidXNlclJvbCI6IkFETUlOIiwiZW1haWxVc2VyIjoicmVjbHV0YWRvci5xYUB5b3BtYWlsLmNvbSIsImtleVN5c3RlbSI6Ik1FWCIsImlhdCI6MTY4NzI3Mjk5MCwiZXhwIjoxNjg3MjgzNzkwfQ._t-iGP0Ivx3L-367vLBBWFVZjlNOrzyBjTkuOFl2A2FTw5W95IlOX68Puk6o_WfhkKBcwXgawcglhXhJvNSDyA</value>
      <webElementGuid>4e882c10-e2f5-4734-9521-6ccce89b8e15</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>DELETE</restRequestMethod>
   <restUrl>${url}/management/dashboard/users/delete-by-email?email=${email}&amp;sendNotification=true</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.email</defaultValue>
      <description></description>
      <id>3c480a35-8d70-4e66-9ee7-086007c84f9d</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>bf10cbab-a569-4a4b-9e23-357d07fd748e</id>
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
